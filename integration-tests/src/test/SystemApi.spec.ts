// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

describe('System API', (): void => {
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

  it('rpc.system.chain() works', (): void => {
    return api.rpc.system.chain().then(result => {
console.log(`**** DEBUG: chain=${result}===`);
      expect(result).toEqual("Development");
    });
  });


  it('can retrieve info via RPC', async (): void => {
    const [chain, nodeName, nodeVersion] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version()
    ]);
    console.log(`**** You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
  });
});
