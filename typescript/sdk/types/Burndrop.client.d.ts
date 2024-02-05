/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate';
import { Coin, StdFee } from '@cosmjs/amino';
import { Uint128, SwapRound, OutputTokenMapForUint128, UpdateRoundParams, OrderBy, Config, PriceResponse, RoundsResponse, SimulateBurnResponse, UserInfoResponse, UsersInfoResponse } from './Burndrop.types';
export interface BurndropReadOnlyInterface {
    contractAddress: string;
    config: () => Promise<Config>;
    userInfo: ({ address }: {
        address: string;
    }) => Promise<UserInfoResponse>;
    usersInfo: ({ limit, order, start, }: {
        limit?: number;
        order?: OrderBy;
        start?: string;
    }) => Promise<UsersInfoResponse>;
    currentPrice: () => Promise<PriceResponse>;
    simulateBurn: ({ amount, }: {
        amount: Uint128;
    }) => Promise<SimulateBurnResponse>;
    rounds: () => Promise<RoundsResponse>;
}
export declare class BurndropQueryClient implements BurndropReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    config: () => Promise<Config>;
    userInfo: ({ address, }: {
        address: string;
    }) => Promise<UserInfoResponse>;
    usersInfo: ({ limit, order, start, }: {
        limit?: number | undefined;
        order?: OrderBy | undefined;
        start?: string | undefined;
    }) => Promise<UsersInfoResponse>;
    currentPrice: () => Promise<PriceResponse>;
    simulateBurn: ({ amount, }: {
        amount: Uint128;
    }) => Promise<SimulateBurnResponse>;
    rounds: () => Promise<RoundsResponse>;
}
export interface BurndropInterface extends BurndropReadOnlyInterface {
    contractAddress: string;
    sender: string;
    burnUusd: ({ amount, minAmountOut, referrer, }: {
        amount: Uint128;
        minAmountOut?: OutputTokenMapForUint128;
        referrer?: string;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    registerStartingUser: ({ user, }: {
        user: string;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    register2ndReferrer: ({ referrer, }: {
        referrer: string;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    updateSlotSize: ({ slotSize, }: {
        slotSize: Uint128;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    createRound: ({ round, }: {
        round: SwapRound;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    updateRound: ({ params, }: {
        params: UpdateRoundParams;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    deleteRound: ({ id, }: {
        id: number;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class BurndropClient extends BurndropQueryClient implements BurndropInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    burnUusd: ({ amount, minAmountOut, referrer, }: {
        amount: Uint128;
        minAmountOut?: OutputTokenMapForUint128 | undefined;
        referrer?: string | undefined;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    registerStartingUser: ({ user, }: {
        user: string;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    register2ndReferrer: ({ referrer, }: {
        referrer: string;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    updateSlotSize: ({ slotSize, }: {
        slotSize: Uint128;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    createRound: ({ round, }: {
        round: SwapRound;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    updateRound: ({ params, }: {
        params: UpdateRoundParams;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
    deleteRound: ({ id, }: {
        id: number;
    }, fee?: number | StdFee | 'auto', memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Burndrop.client.d.ts.map