import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice } from '@cosmjs/stargate';
import {
  type ExecuteMsg,
  calculateBurnFee,
  encodeExecuteMsg,
  getGasPrice,
  sdk,
  trySimulateEncodedMsg,
} from '@mint-cash/burndrop-sdk';

import { config } from '../utils/config';

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();
  console.log({ sender });

  const client = await SigningCosmWasmClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { gasPrice: await getGasPrice() },
  );

  const block = await client.getBlock();
  console.log(block.header.height, block.header.chainId);

  const burndropQueryClient = new sdk.Burndrop.BurndropQueryClient(
    client,
    config.contractAddress,
  );
  const userInfo = await burndropQueryClient.userInfo({ address: sender });
  console.log(userInfo);

  const {
    rounds: [round],
  } = await burndropQueryClient.rounds();
  console.log(round);

  const msg: ExecuteMsg = {
    burn_uusd: {
      amount: (1 * 10 ** 6).toString(), // 1 USTC
      referrer: 'terra13wm0x7mtal0nrx80vmckad50tagak3p4v7fv3z', // self-ref if script 2 is run
    },
  };
  const executeMsg = encodeExecuteMsg({
    contract: config.contractAddress,
    sender,
    msg,
    funds: [{ denom: 'uusd', amount: msg.burn_uusd.amount }],
  });
  const gasInfo = await trySimulateEncodedMsg({
    sender,
    encodedMsg: executeMsg,
    signingCosmwasmClient: client,
  });
  console.log(gasInfo);

  const calculatedFee = await calculateBurnFee(
    gasInfo?.gasUsed,
    msg.burn_uusd.amount,
  );
  const executeResult = await client.signAndBroadcast(
    sender,
    [executeMsg],
    calculatedFee,
  );
  console.log(executeResult);
  console.log(executeResult.gasUsed, executeResult.gasWanted);
}

main().catch(console.error);
