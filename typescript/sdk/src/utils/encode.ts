import { toUtf8 } from '@cosmjs/encoding';
import { Uint53 } from '@cosmjs/math';
import { MsgSend } from 'cosmjs-types/cosmos/bank/v1beta1/tx';
import {
  MsgExecuteContract,
  MsgInstantiateContract,
} from 'cosmjs-types/cosmwasm/wasm/v1/tx';
import { MsgMigrateContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx';

import { ExecuteMsg, MigrateMsg } from '../contracts/Burndrop.types';
import { InstantiateMsg } from '../contracts/Burndrop.types';

export type Fund = { denom: string; amount: string };

export type EncodeSendMsgProps = {
  fromAddress: string;
  toAddress: string;
  amount: { denom: string; amount: string }[];
};
export const encodeSendMsg = ({
  fromAddress,
  toAddress,
  amount,
}: EncodeSendMsgProps) => ({
  typeUrl: '/cosmos.bank.v1beta1.MsgSend',
  value: MsgSend.fromPartial({ fromAddress, toAddress, amount }),
});

export type EncodeInstantiateMsgProps = {
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

export type EncodeExecuteMsgProps = {
  sender: string;
  contract: string;
  msg: ExecuteMsg;
  funds: Fund[];
};
export const encodeExecuteMsg = ({
  sender,
  contract,
  msg,
  funds,
}: EncodeExecuteMsgProps) => ({
  typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
  value: MsgExecuteContract.fromPartial({
    sender,
    contract,
    msg: toUtf8(JSON.stringify(msg)),
    funds,
  }),
});

export type EncodeMigrateMsgProps = {
  sender: string;
  codeId: number;
  contract: string;
  msg: MigrateMsg;
};

export const encodeMigrateMsg = ({
  sender,
  codeId,
  contract,
  msg,
}: EncodeMigrateMsgProps) => ({
  typeUrl: '/cosmwasm.wasm.v1.MsgMigrateContract',
  value: MsgMigrateContract.fromPartial({
    sender,
    contract,
    codeId: BigInt(new Uint53(codeId).toString()),
    msg: toUtf8(JSON.stringify(msg)),
  }),
});
