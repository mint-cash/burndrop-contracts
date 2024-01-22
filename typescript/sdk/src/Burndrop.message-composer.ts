/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { Coin } from '@cosmjs/amino';
import { MsgExecuteContractEncodeObject } from '@cosmjs/cosmwasm-stargate';
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx';
import { toUtf8 } from '@cosmjs/encoding';
import {
  Uint128,
  InstantiateMsg,
  ExecuteMsg,
  QueryMsg,
  Addr,
  Config,
  Decimal,
  PriceResponse,
  SimulateBurnResponse,
  UserInfoResponse,
} from './Burndrop.types';
export interface BurndropMsg {
  contractAddress: string;
  sender: string;
  burnTokens: (
    {
      amount,
      referrer,
    }: {
      amount: Uint128;
      referrer: string;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  registerStartingUser: (
    {
      user,
    }: {
      user: string;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  register2ndReferrer: (
    {
      referrer,
    }: {
      referrer: string;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  updateSlotSize: (
    {
      slotSize,
    }: {
      slotSize: Uint128;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
}
export class BurndropMsgComposer implements BurndropMsg {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.burnTokens = this.burnTokens.bind(this);
    this.registerStartingUser = this.registerStartingUser.bind(this);
    this.register2ndReferrer = this.register2ndReferrer.bind(this);
    this.updateSlotSize = this.updateSlotSize.bind(this);
  }

  burnTokens = (
    {
      amount,
      referrer,
    }: {
      amount: Uint128;
      referrer: string;
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            burn_tokens: {
              amount,
              referrer,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  registerStartingUser = (
    {
      user,
    }: {
      user: string;
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            register_starting_user: {
              user,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  register2ndReferrer = (
    {
      referrer,
    }: {
      referrer: string;
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            register2nd_referrer: {
              referrer,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  updateSlotSize = (
    {
      slotSize,
    }: {
      slotSize: Uint128;
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            update_slot_size: {
              slot_size: slotSize,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
}
