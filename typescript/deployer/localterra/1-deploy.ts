import { OfflineSigner } from '@cosmjs/proto-signing';
import { InstantiateMsg } from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice, calculateFee } from '@cosmjs/stargate';
import math from '@cosmjs/math';
import fs from 'fs';
import amino from '@cosmjs/amino';
import findWorkspaceRoot from 'find-yarn-workspace-root';
import path from 'path';
import { Secp256k1HdWallet, Secp256k1Wallet } from '@cosmjs/amino';
import { stringToPath } from '@cosmjs/crypto';
import encoding from '@cosmjs/encoding';
import tx_4 from 'cosmjs-types/cosmwasm/wasm/v1/tx';

const chainId = 'localterra' as const;
type ConfigArgs = {
  mnemonic?: string;
  privateKey?: string;
  endpoint: string;
};

class Config {
  prefix = 'terra';

  constructor(public args: ConfigArgs) {}

  async getSigner(): Promise<OfflineSigner> {
    let signer: Secp256k1HdWallet | Secp256k1Wallet | undefined = undefined;

    if (this.args.mnemonic) {
      signer = await Secp256k1HdWallet.fromMnemonic(this.args.mnemonic, {
        prefix: this.prefix,
        hdPaths: [stringToPath("m/44'/330'/0'/0/0")],
      });
    } else if (this.args.privateKey) {
      signer = await Secp256k1Wallet.fromKey(
        Buffer.from(this.args.privateKey, 'hex'),
        this.prefix,
      );
    }

    if (!signer) {
      throw Error('no mnemonic or privkey');
    }

    return signer;
  }

  get command(): string {
    return `terrad --node ${this.args.endpoint} --chain-id ${chainId}`;
  }
}

const config = new Config({
  endpoint: process.env.ENDPOINT || 'http://localhost:26657',
  mnemonic: process.env.MNEMONIC,
  privateKey: process.env.PRIVATE_KEY,
});

const YARN_WORKSPACE_ROOT = findWorkspaceRoot();

let WASM_PATH = path.join(
  YARN_WORKSPACE_ROOT!,
  'artifacts',
  'burndrop_contracts.wasm',
);
// if WASM_PATH doesn't exist, try `burndrop_contracts-aarch64.wasm`
if (!fs.existsSync(WASM_PATH)) {
  WASM_PATH = path.join(
    YARN_WORKSPACE_ROOT!,
    'artifacts',
    'burndrop_contracts-aarch64.wasm',
  );
}

async function main() {
  if (!config.args.mnemonic && !config.args.privateKey) {
    console.error(
      'Error: Either mnemonic or privateKey must be provided via process.env',
    );
    return;
  }

  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const client = await SigningCosmWasmClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { gasPrice: GasPrice.fromString('0.02uluna') },
  );

  const wasm = fs.readFileSync(WASM_PATH);
  const uploadResult = await client.upload(sender, wasm, 'auto');
  console.log(uploadResult);

  const instantiateMsg: InstantiateMsg = {
    initial_slot_size: (1000 * 10 ** 6).toString(),
    rounds: [
      {
        id: 1,
        start_time: Math.floor(Date.now() / 1000),
        end_time: Math.floor(Date.now() / 1000) + 60 * 60 * 24 * 7,
        oppamint_liquidity: {
          x: '100000000',
          y: '50000000',
        },
        ancs_liquidity: {
          x: '3000000000',
          y: '200000000',
        },
      },
    ],
    max_query_limit: 30,
    default_query_limit: 10,
  };

  const instantiateContractMsg = {
    typeUrl: '/cosmwasm.wasm.v1.MsgInstantiateContract',
    value: tx_4.MsgInstantiateContract.fromPartial({
      sender,
      codeId: BigInt(new math.Uint53(uploadResult.codeId).toString()),
      label: 'burndrop',
      msg: encoding.toUtf8(JSON.stringify(instantiateMsg)),
      funds: [],
      admin: sender,
    }),
  };

  const accountFromSigner = (await signer.getAccounts()).find(
    (account) => account.address === sender,
  )!;
  const pubkey = amino.encodeSecp256k1Pubkey(accountFromSigner.pubkey);
  const anyMsgs = [instantiateContractMsg].map((m) =>
    client.registry.encodeAsAny(m),
  );
  const { sequence } = await client.getSequence(sender);
  // @ts-ignore
  const queryClient = client.forceGetQueryClient();
  const { gasInfo } = await queryClient.tx.simulate(
    anyMsgs,
    '',
    pubkey,
    sequence,
  );
  console.log(gasInfo);

  const gasEstimation = math.Uint53.fromString(
    gasInfo?.gasUsed.toString() || '0',
  ).toNumber();
  const multiplier = 2.3;
  const usedFee = calculateFee(
    Math.round(gasEstimation * multiplier),
    GasPrice.fromString('0.02uluna'),
  );

  const instantiateResult = await client.instantiate(
    sender,
    uploadResult.codeId,
    instantiateMsg,
    'burndrop',
    usedFee,
    { admin: sender },
  );
  console.log(instantiateResult);
  console.log(`Deployed at ${instantiateResult.contractAddress}`);
}

main().catch(console.error);
