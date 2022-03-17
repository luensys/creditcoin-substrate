# Integration tests for Creditcoin Substrate node

## Requirements

```bash
sudo apt-get install nodejs npm
sudo npm install -g yarn
yarn install --immutable
```

**WARNING:** Node.js 14.x || 16.x is required


## Getting Started

1. Build the software under test, see **Single-Node Development Chain** in `../README.md`
   and execute it locally:

```bash
./target/debug/creditcoin-node --dev
```

2. Execute this test suite:

```bash
yarn test
```
