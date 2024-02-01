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
