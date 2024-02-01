# burndrop-contracts

![npm Downloads](https://img.shields.io/npm/dt/%40mint-cash%2Fburndrop-sdk)

## 🚀

### SDK

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

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

#### LocalTerra ([mint-cash/LocalTerra](https://github.com/mint-cash/LocalTerra))

##### Run the network

```bash
cd ~
git clone https://github.com/mint-cash/LocalTerra
cd LocalTerra

docker-compose up
```

##### Deploy to LocalTerra

```bash
cd typescript/sdk

# run deploy script
# yarn start localterra/1-deploy.ts

# when deploying to LocalTerra (pass creds via process.env)
MNEMONIC="notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" yarn start localterra/1-deploy.ts
PRIVATE_KEY="..." yarn start localterra/1-deploy.ts
```

#### Mainnet

##### Deploy to Mainnet

WIP

```bash
cd typescript/sdk

# run deploy script
# yarn start localterra/1-deploy.ts

# when deploying to mainnet (use .env files for security, WIP)
```
