/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */
import { Coin, StdFee } from '@cosmjs/amino';
import {
  CosmWasmClient,
  ExecuteResult,
  SigningCosmWasmClient,
} from '@cosmjs/cosmwasm-stargate';

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
  OverriddenBurnedUusdResponse,
  OverriddenRound,
  OverriddenRoundsResponse,
  PriceResponse,
  QueryMsg,
  RoundsResponse,
  SimulateBurnResponse,
  SwapRound,
  Uint128,
  UpdateOverriddenRoundParams,
  UpdateRoundParams,
  UserBalanceResponse,
  UserInfoResponse,
  UsersInfoResponse,
} from './Burndrop.types';

export interface BurndropReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  userInfo: ({ address }: { address: string }) => Promise<UserInfoResponse>;
  usersInfo: ({
    limit,
    order,
    start,
  }: {
    limit?: number;
    order?: OrderBy;
    start?: string;
  }) => Promise<UsersInfoResponse>;
  currentPrice: () => Promise<PriceResponse>;
  simulateBurn: ({
    amount,
  }: {
    amount: Uint128;
  }) => Promise<SimulateBurnResponse>;
  rounds: () => Promise<RoundsResponse>;
  guildInfo: ({ guildId }: { guildId: number }) => Promise<GuildInfoResponse>;
  userBalance: ({
    address,
  }: {
    address: string;
  }) => Promise<UserBalanceResponse>;
  overriddenRounds: () => Promise<OverriddenRoundsResponse>;
  overriddenBurnedUusd: ({
    address,
    roundIndex,
  }: {
    address: string;
    roundIndex: number;
  }) => Promise<OverriddenBurnedUusdResponse>;
}
export class BurndropQueryClient implements BurndropReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.userInfo = this.userInfo.bind(this);
    this.usersInfo = this.usersInfo.bind(this);
    this.currentPrice = this.currentPrice.bind(this);
    this.simulateBurn = this.simulateBurn.bind(this);
    this.rounds = this.rounds.bind(this);
    this.guildInfo = this.guildInfo.bind(this);
    this.userBalance = this.userBalance.bind(this);
    this.overriddenRounds = this.overriddenRounds.bind(this);
    this.overriddenBurnedUusd = this.overriddenBurnedUusd.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {},
    });
  };
  userInfo = async ({
    address,
  }: {
    address: string;
  }): Promise<UserInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      user_info: {
        address,
      },
    });
  };
  usersInfo = async ({
    limit,
    order,
    start,
  }: {
    limit?: number;
    order?: OrderBy;
    start?: string;
  }): Promise<UsersInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      users_info: {
        limit,
        order,
        start,
      },
    });
  };
  currentPrice = async (): Promise<PriceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      current_price: {},
    });
  };
  simulateBurn = async ({
    amount,
  }: {
    amount: Uint128;
  }): Promise<SimulateBurnResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      simulate_burn: {
        amount,
      },
    });
  };
  rounds = async (): Promise<RoundsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      rounds: {},
    });
  };
  guildInfo = async ({
    guildId,
  }: {
    guildId: number;
  }): Promise<GuildInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      guild_info: {
        guild_id: guildId,
      },
    });
  };
  userBalance = async ({
    address,
  }: {
    address: string;
  }): Promise<UserBalanceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      user_balance: {
        address,
      },
    });
  };
  overriddenRounds = async (): Promise<OverriddenRoundsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      overridden_rounds: {},
    });
  };
  overriddenBurnedUusd = async ({
    address,
    roundIndex,
  }: {
    address: string;
    roundIndex: number;
  }): Promise<OverriddenBurnedUusdResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      overridden_burned_uusd: {
        address,
        round_index: roundIndex,
      },
    });
  };
}
export interface BurndropInterface extends BurndropReadOnlyInterface {
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
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  registerStartingUser: (
    {
      user,
    }: {
      user: string;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  updateSlotSize: (
    {
      slotSize,
    }: {
      slotSize: Uint128;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  createRound: (
    {
      round,
    }: {
      round: SwapRound;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  updateRound: (
    {
      params,
    }: {
      params: UpdateRoundParams;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  deleteRound: (
    {
      id,
    }: {
      id: number;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  createGuild: (
    {
      name,
      referrer,
    }: {
      name: string;
      referrer?: string;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
  migrateGuild: (
    {
      guildId,
      referrer,
    }: {
      guildId: number;
      referrer?: string;
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
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
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
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
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>;
}
export class BurndropClient
  extends BurndropQueryClient
  implements BurndropInterface
{
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(
    client: SigningCosmWasmClient,
    sender: string,
    contractAddress: string,
  ) {
    super(client, contractAddress);
    this.client = client;
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

  burnUusd = async (
    {
      amount,
      minAmountOut,
      referrer,
    }: {
      amount: Uint128;
      minAmountOut?: OutputTokenMapForUint128;
      referrer?: string;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        burn_uusd: {
          amount,
          min_amount_out: minAmountOut,
          referrer,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  registerStartingUser = async (
    {
      user,
    }: {
      user: string;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        register_starting_user: {
          user,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  updateSlotSize = async (
    {
      slotSize,
    }: {
      slotSize: Uint128;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_slot_size: {
          slot_size: slotSize,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  createRound = async (
    {
      round,
    }: {
      round: SwapRound;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        create_round: {
          round,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  updateRound = async (
    {
      params,
    }: {
      params: UpdateRoundParams;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_round: {
          params,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  deleteRound = async (
    {
      id,
    }: {
      id: number;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        delete_round: {
          id,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  createGuild = async (
    {
      name,
      referrer,
    }: {
      name: string;
      referrer?: string;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        create_guild: {
          name,
          referrer,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  migrateGuild = async (
    {
      guildId,
      referrer,
    }: {
      guildId: number;
      referrer?: string;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        migrate_guild: {
          guild_id: guildId,
          referrer,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  updateOverriddenRound = async (
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
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_overridden_round: {
          end_time: endTime,
          index,
          slot_size: slotSize,
          start_time: startTime,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
  createOverriddenRound = async (
    {
      endTime,
      slotSize,
      startTime,
    }: {
      endTime: number;
      slotSize: Uint128;
      startTime: number;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        create_overridden_round: {
          end_time: endTime,
          slot_size: slotSize,
          start_time: startTime,
        },
      },
      fee,
      memo,
      _funds,
    );
  };
}
