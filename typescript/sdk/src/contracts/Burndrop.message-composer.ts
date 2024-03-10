/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */
import { Coin } from '@cosmjs/amino';
import { MsgExecuteContractEncodeObject } from '@cosmjs/cosmwasm-stargate';
import { toUtf8 } from '@cosmjs/encoding';
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx';

import {
  Addr,
  Config,
  CreateOverriddenRoundParams,
  Decimal,
  ExecuteMsg,
  GuildInfoResponse,
  InstantiateMsg,
  LiquidityPair,
  MigrateMsg,
  OrderBy,
  OutputTokenMapForDecimal,
  OutputTokenMapForUint128,
  PriceResponse,
  QueryMsg,
  RoundsResponse,
  SimulateBurnResponse,
  SwapRound,
  Uint128,
  UpdateOverriddenRoundParams,
  UpdateRoundParams,
  UserInfoResponse,
  UsersInfoResponse,
} from './Burndrop.types';

export interface BurndropMsg {
  contractAddress: string;
  sender: string;
  burnUusd: (
    {
      amount,
      minAmountOut,
      referrer,
    }: {
      amount: Uint128;
      minAmountOut?: OutputTokenMapForUint128;
      referrer?: string;
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
  updateSlotSize: (
    {
      slotSize,
    }: {
      slotSize: Uint128;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  createRound: (
    {
      round,
    }: {
      round: SwapRound;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  updateRound: (
    {
      params,
    }: {
      params: UpdateRoundParams;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  deleteRound: (
    {
      id,
    }: {
      id: number;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  createGuild: (
    {
      name,
      referrer,
    }: {
      name: string;
      referrer?: string;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  migrateGuild: (
    {
      guildId,
      referrer,
    }: {
      guildId: number;
      referrer?: string;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  updateOverriddenRound: (
    {
      endTime,
      index,
      slotSize,
      startTime,
    }: {
      endTime?: number;
      index: number;
      slotSize: Uint128;
      startTime?: number;
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject;
  createOverriddenRound: (
    {
      endTime,
      slotSize,
      startTime,
    }: {
      endTime: number;
      slotSize: Uint128;
      startTime: number;
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
    this.burnUusd = this.burnUusd.bind(this);
    this.registerStartingUser = this.registerStartingUser.bind(this);
    this.updateSlotSize = this.updateSlotSize.bind(this);
    this.createRound = this.createRound.bind(this);
    this.updateRound = this.updateRound.bind(this);
    this.deleteRound = this.deleteRound.bind(this);
    this.createGuild = this.createGuild.bind(this);
    this.migrateGuild = this.migrateGuild.bind(this);
    this.updateOverriddenRound = this.updateOverriddenRound.bind(this);
    this.createOverriddenRound = this.createOverriddenRound.bind(this);
  }

  burnUusd = (
    {
      amount,
      minAmountOut,
      referrer,
    }: {
      amount: Uint128;
      minAmountOut?: OutputTokenMapForUint128;
      referrer?: string;
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
            burn_uusd: {
              amount,
              min_amount_out: minAmountOut,
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
  createRound = (
    {
      round,
    }: {
      round: SwapRound;
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
            create_round: {
              round,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  updateRound = (
    {
      params,
    }: {
      params: UpdateRoundParams;
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
            update_round: {
              params,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  deleteRound = (
    {
      id,
    }: {
      id: number;
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
            delete_round: {
              id,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  createGuild = (
    {
      name,
      referrer,
    }: {
      name: string;
      referrer?: string;
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
            create_guild: {
              name,
              referrer,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  migrateGuild = (
    {
      guildId,
      referrer,
    }: {
      guildId: number;
      referrer?: string;
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
            migrate_guild: {
              guild_id: guildId,
              referrer,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  updateOverriddenRound = (
    {
      endTime,
      index,
      slotSize,
      startTime,
    }: {
      endTime?: number;
      index: number;
      slotSize: Uint128;
      startTime?: number;
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
            update_overridden_round: {
              end_time: endTime,
              index,
              slot_size: slotSize,
              start_time: startTime,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
  createOverriddenRound = (
    {
      endTime,
      slotSize,
      startTime,
    }: {
      endTime: number;
      slotSize: Uint128;
      startTime: number;
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
            create_overridden_round: {
              end_time: endTime,
              slot_size: slotSize,
              start_time: startTime,
            },
          }),
        ),
        funds: _funds,
      }),
    };
  };
}
