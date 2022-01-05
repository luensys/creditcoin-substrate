#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, sp_runtime, traits::OnTimestampSet};
pub use pallet::*;
use primitives::Difficulty;
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::traits::{SaturatedConversion, UniqueSaturatedInto};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DifficultyAndTimestamp<Moment> {
	difficulty: Difficulty,
	timestamp: Moment,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		sp_runtime::traits::{MaybeSerializeDeserialize, SaturatedConversion},
	};
	use frame_system::pallet_prelude::*;
	use primitives::Difficulty;
	use sp_arithmetic::traits::{BaseArithmetic, UniqueSaturatedInto};

	use crate::DifficultyAndTimestamp;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Moment: Parameter
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ scale_info::StaticTypeInfo
			+ SaturatedConversion
			+ BaseArithmetic
			+ UniqueSaturatedInto<i64>
			+ MaybeSerializeDeserialize;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_difficulty: Difficulty,
		pub target_time: T::Moment,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_difficulty: Difficulty::from(1_000_000),
				target_time: T::Moment::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			CurrentDifficulty::<T>::put(self.initial_difficulty);
			TargetBlockTime::<T>::put(self.target_time);
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn previous_difficulties_and_timestamps)]
	pub type PreviousDifficultiesAndTimestamps<T> = StorageValue<
		_,
		BoundedVec<DifficultyAndTimestamp<<T as Config>::Moment>, ConstU32<2>>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn difficulty)]
	pub type CurrentDifficulty<T> = StorageValue<_, Difficulty, ValueQuery>;

	#[pallet::storage]
	pub type TargetBlockTime<T: Config> = StorageValue<_, T::Moment, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_target_block_time(
			origin: OriginFor<T>,
			target_time: T::Moment,
		) -> DispatchResult {
			let _who = ensure_root(origin)?;

			TargetBlockTime::<T>::put(target_time);

			Ok(())
		}
	}
}

macro_rules! if_std {
	($($t: tt)*) => {
		#[cfg(feature = "std")]
		{
			$($t)*
		}
	};
}

// Adapted from zawy12's Simple EMA difficulty algorithm, license follows:
/*
	MIT License

	Copyright (c) 2017 zawy12

	Permission is hereby granted, free of charge, to any person obtaining a copy
	of this software and associated documentation files (the "Software"), to deal
	in the Software without restriction, including without limitation the rights
	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
	copies of the Software, and to permit persons to whom the Software is
	furnished to do so, subject to the following conditions:

	The above copyright notice and this permission notice shall be included in all
	copies or substantial portions of the Software.

	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
	SOFTWARE.
*/
fn next_difficulty<M>(
	previous: &[DifficultyAndTimestamp<M>],
	target_time: M,
	initial_difficulty: Difficulty,
) -> Difficulty
where
	M: SaturatedConversion
		+ Copy
		+ BaseArithmetic
		+ UniqueSaturatedInto<i64>
		+ frame_support::sp_std::fmt::Debug,
{
	const N: i64 = 120;
	if_std! {
		log::info!("previous {:?}", previous);
	}
	if previous.len() < 2 {
		return initial_difficulty
	}

	let oldest = &previous[0];
	let newest = &previous[1];

	let t = target_time.saturated_into::<i64>() / 1000;
	if_std! {
		log::info!("t = {}", t);
	}
	let solve_time = (newest.timestamp.saturated_into::<i64>() -
		oldest.timestamp.saturated_into::<i64>()) /
		1000;

	if_std! {
		log::info!("solve time = {}", solve_time);
	}
	let solve_time = i64::max(-5 * t, i64::min(solve_time, 6 * t));
	if_std! {
		log::info!("ST = {}", solve_time);
	}
	let difficulty = newest.difficulty;

	let next_difficulty = (difficulty * N * t) / (N * t - t + solve_time);

	if_std! {
		log::info!("next difficulty = {}", next_difficulty);
	}

	next_difficulty
}

impl<T: Config> OnTimestampSet<T::Moment> for Pallet<T> {
	fn on_timestamp_set(current_timestamp: T::Moment) {
		let target_time = TargetBlockTime::<T>::get();
		let current_difficulty = Self::difficulty();

		let mut previous = PreviousDifficultiesAndTimestamps::<T>::get();

		let current =
			DifficultyAndTimestamp { difficulty: current_difficulty, timestamp: current_timestamp };

		if previous.len() < 2 {
			previous.try_push(current).expect("len < 2 checked above");
		} else {
			previous[0] = previous[1].clone();
			previous[1] = current;
		}

		let next_difficulty = next_difficulty(&previous, target_time, current_difficulty);
		CurrentDifficulty::<T>::put(next_difficulty);
		PreviousDifficultiesAndTimestamps::<T>::put(previous);
	}
}