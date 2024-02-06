import { ExecuteMsg } from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice, calculateFee } from '@cosmjs/stargate';
import math from '@cosmjs/math';
import amino from '@cosmjs/amino';
import encoding from '@cosmjs/encoding';
import tx_4 from 'cosmjs-types/cosmwasm/wasm/v1/tx';
import { config } from '../utils/config';

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

  const msg: ExecuteMsg = {
    register_starting_user: {
      // user: 'terra17tv2hvwpg0ukqgd2y5ct2w54fyan7z0zxrm2f9',
      user: sender, // self
    },
  };
  const executeMsg = {
    typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
    value: tx_4.MsgExecuteContract.fromPartial({
      sender,
      contract: BURNDROP_CONTRACT_ADDRESS,
      msg: encoding.toUtf8(JSON.stringify(msg)),
      funds: [],
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
  const gasAdjustment = 1.4;
  const usedFee = calculateFee(
    Math.round(gasEstimation * gasAdjustment),
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
