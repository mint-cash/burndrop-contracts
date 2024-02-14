import { InstantiateMsg } from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice } from '@cosmjs/stargate';
import fs from 'fs';
import findWorkspaceRoot from 'find-yarn-workspace-root';
import path from 'path';
import { config } from '../utils/config';
import {
  calculateFee,
  encodeInstantiateMsg,
  trySimulateExecuteMsg,
} from '../cosmos/tx';

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
          x: '50000000',
          y: '100000000',
        },
        ancs_liquidity: {
          x: '200000000',
          y: '3000000000',
        },
        oppamint_weight: 3,
        ancs_weight: 2,
      },
    ],
    max_query_limit: 30,
    default_query_limit: 10,
    genesis_guild_name: 'Genesis Guild',
    genesis_guild_slug: 'genesis-guild',
  };

  const instantiateContractMsg = encodeInstantiateMsg({
    sender,
    msg: instantiateMsg,
    label: 'burndrop',
    codeId: uploadResult.codeId,
  });
  const gasInfo = await trySimulateExecuteMsg({
    sender,
    encodedMsg: instantiateContractMsg,
    signingCosmwasmClient: client,
  });
  console.log(gasInfo);

  const calculatedFee = calculateFee(gasInfo?.gasUsed);
  const instantiateResult = await client.signAndBroadcast(
    sender,
    [instantiateContractMsg],
    calculatedFee,
  );
  const instantiateEvent = instantiateResult.events.find(
    (v) => v.type === 'instantiate',
  );
  const contractAddress = instantiateEvent?.attributes.find(
    (attr) => attr.key === '_contract_address',
  )?.value;
  console.log(`Deployed at ${contractAddress}`);
}

main().catch(console.error);
