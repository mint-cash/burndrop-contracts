import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import {
  type ExecuteMsg,
  calculateFee,
  encodeExecuteMsg,
  getGasPrice,
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

  const msg: ExecuteMsg = {
    register_starting_user: {
      user: 'terra1euqghzv5vra2rdu2krmj8rj58xl7yd9g7eam0u',
      // user: sender, // self
    },
  };
  const executeMsg = encodeExecuteMsg({
    contract: config.contractAddress,
    sender,
    msg,
    funds: [],
  });
  const gasInfo = await trySimulateEncodedMsg({
    sender,
    encodedMsg: executeMsg,
    signingCosmwasmClient: client,
  });
  console.log(gasInfo);

  const calculatedFee = await calculateFee(gasInfo?.gasUsed);
  const executeResult = await client.signAndBroadcast(
    sender,
    [executeMsg],
    calculatedFee,
  );
  console.log(executeResult);
  console.log(executeResult.gasUsed, executeResult.gasWanted);
}

main().catch(console.error);
