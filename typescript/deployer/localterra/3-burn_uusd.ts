import { ExecuteMsg } from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice } from '@cosmjs/stargate';
import { Uint53 } from '@cosmjs/math';
import amino, { coin } from '@cosmjs/amino';
import encoding from '@cosmjs/encoding';
import tx_4 from 'cosmjs-types/cosmwasm/wasm/v1/tx';
import { config } from '../utils/config';
import sdk from '@mint-cash/burndrop-sdk';

const BURNDROP_CONTRACT_ADDRESS =
  process.env.BURNDROP_CONTRACT_ADDRESS ||
  'terra1657pee2jhf4jk8pq6yq64e758ngvum45gl866knmjkd83w6jgn3syqe77g';

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
      amount: (1 * 10 ** 6).toString(), // 600 USTC
      referrer: userInfo.burned === '0' ? sender : undefined, // self-ref if script 2 is run
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

  const gasUsed = Uint53.fromString(
    gasInfo?.gasUsed.toString() || '0',
  ).toNumber();
  const gasAdjustment = 1.4;
  const gasLimit = Math.round(gasUsed * gasAdjustment);

  // 0.01133uluna,0.15uusd
  const gasPrices = [
    GasPrice.fromString('0.01133uluna'),
    GasPrice.fromString('0.15uusd'),
  ];
  const calculatedFee = {
    amount: gasPrices.map(({ amount, denom }) => {
      const fee = amount.multiply(new Uint53(gasLimit)).ceil().toString();
      return coin(fee, denom);
    }),
    gas: gasLimit.toString(),
  };

  const executeResult = await client.signAndBroadcast(
    sender,
    [executeMsg],
    calculatedFee,
  );
  console.log(executeResult);
  console.log(executeResult.gasUsed, executeResult.gasWanted);
}

main().catch(console.error);
