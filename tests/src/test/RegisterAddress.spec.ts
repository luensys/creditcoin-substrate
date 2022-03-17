// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

describe('RegisterAddress', (): void => {
  let api: ApiPromise;

  beforeEach(async () => {
    process.env.NODE_ENV = 'test';

    const provider = new WsProvider('ws://127.0.0.1:9944');

    api = await ApiPromise.create({ provider });

    const keyring = new Keyring({
      type: 'sr25519'
    });
    const alice = keyring.addFromUri('//Alice');
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('should work', (): void => {
    //    console.error(api.rpc.payment.queryFeeDetails.meta);
    //    console.error(api.rx.rpc.chain.getBlock.meta);
  });
});
