import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import {
  type InstantiateMsg,
  calculateFee,
  encodeInstantiateMsg,
  getGasPrice,
  trySimulateEncodedMsg,
} from '@mint-cash/burndrop-sdk';
import findWorkspaceRoot from 'find-yarn-workspace-root';
import fs from 'fs';
import path from 'path';

import { config } from '../utils/config';

const YARN_WORKSPACE_ROOT = findWorkspaceRoot() || '';

let WASM_PATH = path.join(
  YARN_WORKSPACE_ROOT,
  'artifacts',
  'burndrop_contracts.wasm',
);

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();

  const client = await SigningCosmWasmClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { gasPrice: await getGasPrice() },
  );

  const wasm = fs.readFileSync(WASM_PATH);
  const uploadResult = await client.upload(sender, wasm, 'auto');
  console.log(uploadResult);

  // 2024.02.29 22:45:00 GMT+9 (KST) = 2024.02.29 13:45:00 UTC
  const startTime = Math.floor(
    new Date('2024-02-29T13:45:00Z').getTime() / 1000,
  );

  // endTime = startTime + 30 minutes
  const endTime = startTime + 60 * 30;

  const instantiateMsg: InstantiateMsg = {
    initial_slot_size: (1000 * 10 ** 6).toString(), // 1000 USTC
    rounds: [
      {
        id: 1,
        start_time: startTime,
        end_time: endTime,
        oppamint_liquidity: {
          x: (28_125_000n * 10n ** 6n).toString(),
          y: (18_750_000n * 10n ** 6n).toString(),
        },
        ancs_liquidity: {
          x: (9_375_000n * 10n ** 6n).toString(),
          y: (9_375_000n * 10n ** 6n).toString(),
        },
        oppamint_weight: 3,
        ancs_weight: 2,
      },
    ],
    max_query_limit: 30,
    default_query_limit: 10,
    genesis_guild_name: 'Genesis Guild',
  };

  const instantiateContractMsg = encodeInstantiateMsg({
    sender,
    msg: instantiateMsg,
    label: 'burndrop',
    codeId: uploadResult.codeId,
  });
  const gasInfo = await trySimulateEncodedMsg({
    sender,
    encodedMsg: instantiateContractMsg,
    signingCosmwasmClient: client,
  });
  console.log(gasInfo);

  const calculatedFee = await calculateFee(gasInfo?.gasUsed);
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
  console.log(
    `Deployed to contract: ${contractAddress} in block height: ${instantiateResult.height}`,
  );
}

main().catch(console.error);
