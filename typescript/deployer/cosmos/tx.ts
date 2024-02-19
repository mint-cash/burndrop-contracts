import { encodeSecp256k1Pubkey } from '@cosmjs/amino';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { toUtf8 } from '@cosmjs/encoding';
import { Uint53 } from '@cosmjs/math';
import { EncodeObject, type AccountData } from '@cosmjs/proto-signing';
import { GasPrice, coin } from '@cosmjs/stargate';
import {
  ExecuteMsg,
  InstantiateMsg,
} from '@mint-cash/burndrop-sdk/types/Burndrop.types';
import {
  MsgExecuteContract,
  MsgInstantiateContract,
} from 'cosmjs-types/cosmwasm/wasm/v1/tx';

import { config } from '../utils/config';

type EncodeInstantiateMsgProps = {
  sender: string;
  msg: InstantiateMsg;
  label: string;
  codeId: number;
};
export const encodeInstantiateMsg = ({
  sender,
  msg,
  label,
  codeId,
}: EncodeInstantiateMsgProps) => ({
  typeUrl: '/cosmwasm.wasm.v1.MsgInstantiateContract',
  value: MsgInstantiateContract.fromPartial({
    sender,
    codeId: BigInt(new Uint53(codeId).toString()),
    label,
    msg: toUtf8(JSON.stringify(msg)),
    funds: [],
    admin: sender,
  }),
});

type EncodeExecuteMsgProps = {
  sender: string;
  msg: ExecuteMsg;
  funds: { denom: string; amount: string }[];
};
export const encodeExecuteMsg = ({
  sender,
  msg,
  funds,
}: EncodeExecuteMsgProps) => ({
  typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
  value: MsgExecuteContract.fromPartial({
    sender,
    contract: config.contractAddress,
    msg: toUtf8(JSON.stringify(msg)),
    funds,
  }),
});

type TrySimulateExecuteMsgProps = {
  sender: string;
  encodedMsg: EncodeObject;
  signingCosmwasmClient: SigningCosmWasmClient;
};
export const trySimulateExecuteMsg = async ({
  sender,
  encodedMsg,
  signingCosmwasmClient,
}: TrySimulateExecuteMsgProps) => {
  try {
    const anyMsgs = [encodedMsg].map((m) =>
      signingCosmwasmClient.registry.encodeAsAny(m),
    );

    const accountFromSigner = // @ts-ignore
      (await signingCosmwasmClient.signer.getAccounts()).find(
        (account: AccountData) => account.address === sender,
      )!;
    const pubkey = encodeSecp256k1Pubkey(accountFromSigner.pubkey);
    const { sequence } = await signingCosmwasmClient.getSequence(sender);

    // @ts-ignore
    const queryClient = signingCosmwasmClient.forceGetQueryClient();
    const { gasInfo } = await queryClient.tx.simulate(
      anyMsgs,
      '',
      pubkey,
      sequence,
    );
    return gasInfo || null;
  } catch (err) {
    console.error('[!] Simulation Error', err);
    return null;
  }
};

export const DEFAULT_GAS = '300000'; // 300K
export const DEFAULT_GAS_ADJUSTMENT = 1.4;

// 0.01133uluna,0.15uusd
export const DEFAULT_GAS_PRICES = [
  GasPrice.fromString('0.01133uluna'),
  GasPrice.fromString('0.15uusd'),
];

export const calculateFee = (
  estimatedGasUsed: bigint | undefined,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
  gasPrices: GasPrice[] = DEFAULT_GAS_PRICES,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);

  return {
    amount: gasPrices.map(({ amount, denom }) => {
      const fee = amount.multiply(new Uint53(gasLimit)).ceil().toString();
      return coin(fee, denom);
    }),
    gas: gasLimit.toString(),
  };
};

export const calculateBurnFee = (
  estimatedGasUsed: bigint | undefined,
  burnAmount: string,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
  gasPrices: GasPrice[] = DEFAULT_GAS_PRICES,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);
  const gasLimitFromBurnAmount = Math.round(
    Uint53.fromString(burnAmount).toNumber() * 0.0233 * gasAdjustment,
  );

  // 0.0233 = 0.0035 / 0.15
  return {
    amount: gasPrices.map(({ amount, denom }) => {
      const fee = amount
        .multiply(new Uint53(Math.max(gasLimit, gasLimitFromBurnAmount)))
        .ceil()
        .toString();
      return coin(fee, denom);
    }),
    gas: gasLimit.toString(),
  };
};
