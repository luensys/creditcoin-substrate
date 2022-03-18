// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

describe('RegisterAddress', (): void => {
  let api: ApiPromise;
  let alice: KeyringPair;

  beforeEach(async () => {
    process.env.NODE_ENV = 'test';

    const provider = new WsProvider('ws://127.0.0.1:9944');

    api = await ApiPromise.create({ provider });

    const keyring = new Keyring({
      type: 'sr25519'
    });
    const alice = keyring.addFromUri('//Alice', { name: 'Alice' });
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('should work', async (): void => {
    const unsub = await api.tx.creditcoin.registerAddress('Luniverse', '0xafafafafafafafafafafaf')
        .signAndSend(alice, ({
            status,
            events = []
        }) => {
            if (status.isInBlock) {
                let registered = events.find(({
                    event: {
                        section,
                        method
                    }
                }) => {
                    return section == 'creditcoin' && method == 'AddressRegistered'
                });

                expect(registered).toBe(true);

                let addressId = registered.event.data[0].toString();
                let address = registered.event.data[1].toString();
                console.log(`registered address ${address} with id = ${addressId}`);

                unsub();
            }
        });

//    console.error(unsub);
  });
});
