# burndrop-contracts

![npm Downloads](https://img.shields.io/npm/dt/%40mint-cash%2Fburndrop-sdk)

<!-- initiatives here -->

## 🚀

### Build & Codegen for SDK

```bash
# Generate schema
cargo build
cargo schema

# Install dependencies
yarn

# Run codegen
yarn workspace @mint-cash/burndrop-sdk codegen

# Build TypeScript
yarn workspace @mint-cash/burndrop-sdk build

# Publish to npm
cd typescript/sdk
yarn npm publish --access=public
```

### Deploy

Before uploading the contract, please generate an optimized version of the production code using the specified `cosmwasm/rust-optimizer` version:

```bash
# x64
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11

# arm64
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.11
```

#### Local Network

##### Run the network

Using [mint-cash/LocalTerra](https://github.com/mint-cash/LocalTerra):

```bash
cd ~
git clone https://github.com/mint-cash/LocalTerra
cd LocalTerra

docker-compose up
```

##### Deploy to local network

```bash
cd typescript/deployer

# run deploy script
# yarn start localterra/1-deploy.ts

# when using LocalTerra
MNEMONIC="..." yarn start localterra/1-deploy.ts
PRIVATE_KEY="..." yarn start localterra/1-deploy.ts
```
