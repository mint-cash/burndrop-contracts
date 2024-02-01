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

#### Local Network

##### Run the network

If using [mint-cash/LocalTerra](https://github.com/mint-cash/LocalTerra):

```bash
cd ~
git clone https://github.com/mint-cash/LocalTerra
cd LocalTerra

docker-compose up
```

If using [classic-terra/core](https://github.com/classic-terra/core)'s localnet:

```bash
cd ~
git clone https://github.com/classic-terra/core terra-classic-core
cd terra-classic-core

make localnet-start
systemctl restart docker
docker-compose up -d
```

##### Deploy to local network

```bash
cd typescript/sdk

# run deploy script
# yarn start localterra/1-deploy.ts

# when using LocalTerra (pass creds via process.env)
MNEMONIC="notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" yarn start localterra/1-deploy.ts
PRIVATE_KEY="..." yarn start localterra/1-deploy.ts

# when using localnet
MNEMONIC="typical hood basket desert stumble outside brisk blind total setup disorder side oblige engage prison wink reopen above welcome resource decade flight praise later" yarn start localterra/1-deploy.ts
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
