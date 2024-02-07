import { ExecuteMsg } from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice } from '@cosmjs/stargate';

import { config } from '../utils/config';
import {
  calculateFee,
  encodeExecuteMsg,
  trySimulateExecuteMsg,
} from '../cosmos/tx';

async function main() {
  const signer = await config.getSigner();
  const [{ address: sender }] = await signer.getAccounts();
  console.log({ sender });

  const client = await SigningCosmWasmClient.connectWithSigner(
    config.args.endpoint,
    signer,
    { gasPrice: GasPrice.fromString('0.02uluna') },
  );

  const block = await client.getBlock();
  console.log(block.header.height, block.header.chainId);

  const msg: ExecuteMsg = {
    register_starting_user: {
      // user: 'terra17tv2hvwpg0ukqgd2y5ct2w54fyan7z0zxrm2f9',
      user: sender, // self
    },
  };
  const executeMsg = encodeExecuteMsg({
    sender,
    msg,
    funds: [],
  });
  const gasInfo = await trySimulateExecuteMsg({
    sender,
    encodedMsg: executeMsg,
    signingCosmwasmClient: client,
  });
  console.log(gasInfo);

  const calculatedFee = calculateFee(gasInfo?.gasUsed);
  const executeResult = await client.signAndBroadcast(
    sender,
    [executeMsg],
    calculatedFee,
  );
  console.log(executeResult);
  console.log(executeResult.gasUsed, executeResult.gasWanted);
}

main().catch(console.error);
