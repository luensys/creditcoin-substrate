use std::ops::Deref;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_runtime::traits::UniqueSaturatedInto;

use super::ExternalAmount;

pub type InterestRate = u64;

pub const INTEREST_RATE_PRECISION: u64 = 10_000;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LoanTerms<Moment> {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub maturity: Moment,
}

fn calc_interest(principal_amount: &ExternalAmount, interest_rate: InterestRate) -> ExternalAmount {
	principal_amount * interest_rate / INTEREST_RATE_PRECISION
}

impl<Moment> LoanTerms<Moment> {
	pub fn calc_interest(&self) -> ExternalAmount {
		calc_interest(&self.amount, self.interest_rate)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskTerms<Moment>(LoanTerms<Moment>);

impl<Moment> Deref for AskTerms<Moment> {
	type Target = LoanTerms<Moment>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<Moment> From<LoanTerms<Moment>> for AskTerms<Moment> {
	fn from(terms: LoanTerms<Moment>) -> Self {
		Self(terms)
	}
}

impl<Moment> AskTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	pub fn match_with(&self, bid_terms: &BidTerms<Moment>) -> bool {
		self.amount == bid_terms.amount
			&& (self.interest_rate / self.maturity.unique_saturated_into())
				>= (bid_terms.interest_rate / bid_terms.maturity.unique_saturated_into())
	}

	pub fn agreed_terms(&self, bid_terms: BidTerms<Moment>) -> Option<LoanTerms<Moment>> {
		self.match_with(&bid_terms).then(|| bid_terms.0)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidTerms<Moment>(LoanTerms<Moment>);

impl<Moment> Deref for BidTerms<Moment> {
	type Target = LoanTerms<Moment>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<Moment> From<LoanTerms<Moment>> for BidTerms<Moment> {
	fn from(terms: LoanTerms<Moment>) -> BidTerms<Moment> {
		Self(terms)
	}
}

impl<Moment> BidTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	pub fn match_with(&self, ask_terms: &AskTerms<Moment>) -> bool {
		ask_terms.match_with(self)
	}

	pub fn agreed_terms(self, ask_terms: &AskTerms<Moment>) -> Option<LoanTerms<Moment>> {
		ask_terms.agreed_terms(self)
	}
}

#[cfg(test)]
mod tests {
	use super::calc_interest;
	use crate::ExternalAmount;
	use ethereum_types::U256;

	#[test]
	pub fn test_calc_interest() {
		let principal_amount = ExternalAmount::from(100_000u64);
		let interest_rate_bps = 1_000;
		let interest = calc_interest(&principal_amount, interest_rate_bps);
		assert_eq!(interest, U256::from(10_000u64));
	}
}