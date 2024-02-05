import { ExecuteMsg } from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice, calculateFee } from '@cosmjs/stargate';
import math from '@cosmjs/math';
import amino from '@cosmjs/amino';
import encoding from '@cosmjs/encoding';
import tx_4 from 'cosmjs-types/cosmwasm/wasm/v1/tx';
import { config } from '../utils/config';
import sdk from '@mint-cash/burndrop-sdk';

const BURNDROP_CONTRACT_ADDRESS =
  'terra13we0myxwzlpx8l5ark8elw5gj5d59dl6cjkzmt80c5q5cv5rt54qgeyjkp';

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

  const burndropQueryClient = new sdk.contracts.Burndrop.BurndropQueryClient(
    client,
    BURNDROP_CONTRACT_ADDRESS,
  );
  const userInfo = await burndropQueryClient.userInfo({ address: sender });
  console.log(userInfo);

  const {
    rounds: [round],
  } = await burndropQueryClient.rounds();
  console.log(round);

  const msg: ExecuteMsg = {
    burn_uusd: {
      amount: (600 * 10 ** 6).toString(), // 600 USTC
      referrer: undefined,
    },
  };
  const executeMsg = {
    typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
    value: tx_4.MsgExecuteContract.fromPartial({
      sender,
      contract: BURNDROP_CONTRACT_ADDRESS,
      msg: encoding.toUtf8(JSON.stringify(msg)),
      funds: [{ denom: 'uusd', amount: msg.burn_uusd.amount }],
    }),
  };

  const accountFromSigner = (await signer.getAccounts()).find(
    (account) => account.address === sender,
  )!;
  console.log({ accountFromSigner });
  const pubkey = amino.encodeSecp256k1Pubkey(accountFromSigner.pubkey);
  console.log(msg, executeMsg);
  const anyMsgs = [executeMsg].map((m) => client.registry.encodeAsAny(m));
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

  const executeResult = await client.signAndBroadcast(
    sender,
    [executeMsg],
    usedFee,
  );
  console.log(executeResult);
  console.log(executeResult.gasUsed, executeResult.gasWanted);
}

main().catch(console.error);
