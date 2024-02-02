/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import {
  CosmWasmClient,
  SigningCosmWasmClient,
  ExecuteResult,
} from '@cosmjs/cosmwasm-stargate';
import { Coin, StdFee } from '@cosmjs/amino';
import {
  Uint128,
  InstantiateMsg,
  SwapRound,
  LiquidityPair,
  ExecuteMsg,
  UpdateRoundParams,
  QueryMsg,
  OrderBy,
  MigrateMsg,
  Addr,
  Config,
  Decimal,
  PriceResponse,
  OutputTokenMapForDecimal,
  RoundsResponse,
  SimulateBurnResponse,
  OutputTokenMapForUint128,
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
}
export interface BurndropInterface extends BurndropReadOnlyInterface {
  contractAddress: string;
  sender: string;
  burnUusd: (
    {
      amount,
      referrer,
    }: {
      amount: Uint128;
      referrer: string;
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
  register2ndReferrer: (
    {
      referrer,
    }: {
      referrer: string;
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
    this.register2ndReferrer = this.register2ndReferrer.bind(this);
    this.updateSlotSize = this.updateSlotSize.bind(this);
    this.createRound = this.createRound.bind(this);
    this.updateRound = this.updateRound.bind(this);
    this.deleteRound = this.deleteRound.bind(this);
  }

  burnUusd = async (
    {
      amount,
      referrer,
    }: {
      amount: Uint128;
      referrer: string;
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
  register2ndReferrer = async (
    {
      referrer,
    }: {
      referrer: string;
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        register2nd_referrer: {
          referrer,
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
}
