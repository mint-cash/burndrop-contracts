# Mint Cash Burndrop Contracts

![npm Downloads](https://img.shields.io/npm/dt/%40mint-cash%2Fburndrop-sdk)

The Burndrop contract implements logic on [CosmWasm](https://cosmwasm.com) for a Cosmos SDK chain, like Terra Classic, to distribute tokens using a vAMM model for a fixed number of tokens burnt.

If you are new to Mint Cash's Burndrop program, read through the [Burndrop Guide](https://burndrop-docs.mintca.sh/) first.

## Basic Concepts

The Burndrop consists of **slots**, **rounds**, and **virtual liquidity**.

**Slots** define the maximum amount of TerraClassicUSD (USTC) that can be burnt at a time. As the user refers more users, the **burndrop multiplier** is applied, which doubles the number of slots available per **epoch**. This is set to **1,000 USTC** on launch.

**Rounds** define the current **virtual liquidity state** for oppaMINT and ANCS tokens being distributed through the Burndrop program; as every round passes, virtual liquidity halves -- and, therefore, the amount of tokens available for distrubution.

Currently, this is set to **30 days**.

**Virtual liquidity** means liquidity parameters under a simulated Uniswap-like `xyk` curve for purposes of distribution. As more USTC is burnt, this is processed as a "swap" from USTC to oppaMINT and ANCS on both pools, therefore increasing the amount of USTC required for 1 oppaMINT and ANCS.

## Contract Specifications

### `QueryMsg`

- `UserInfo`: accepts an `address` of the User as a `String`. Returns the following:

  - `burned` (`uint128`): the amount of USTC burnt for this User.
  - `burnable` (`uint128`): remaining USTC burn cap for this User.
  - `cap` (`uint128`): total USTC that can be burnt by this User.
  - `slots` (`uint128`): total number of slots earned by this user through means of referral.
  - `slot_size` (`uint128`): the current slot size for this user, denominated in USTC.
  - `swapped_out` (`OutputTokenMap<uint128>`): returns oppaMINT and ANCS tokens earned for this User as a `Map` data structure.
  - `guild_id` (`u64`): the Guild ID for this user.
  - `guild_contributed_uusd` (`uint128`): the amount of USTC burnt by this User that counts for total Guild Statistics.

- `UsersInfo`: a lookup message with `start`, `limit`, and `order` parameters. Returns a `Vector` of `UserInfoResponse`s, as documented above.

- `CurrentPrice`: returns a `PriceResponse`

  - `price` (`OutputTokenMap<uint128>`): returns oppaMINT and ANCS token values denominated in USTC.

- `SimulateBurn`: accepts an `amount` of USTC tokens to be burnt. Returns the following:

  - `swapped_out` (`OutputTokenMap<uint128>`): returns oppaMINT and ANCS tokens to be earned as a result of burnt USTC.
  - `final_amount`: returns USTC tokens to be finally burnt.

- `Rounds`: returns all current and past Rounds.

  - `rounds`(`Vec<SwapRound>`)

- `GuildInfo`: returns information about a Guild with ID `guild_id`.
  - `burned_uusd`: total USTC tokens burnt by this Guild.

<!-- initiatives here -->

## Building & Deploying Contracts

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
