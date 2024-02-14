/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */
import { Coin } from '@cosmjs/amino';
import { MsgExecuteContractEncodeObject } from '@cosmjs/cosmwasm-stargate';
import { Uint128, SwapRound, OutputTokenMapForUint128, UpdateRoundParams } from './Burndrop.types';
export interface BurndropMsg {
    contractAddress: string;
    sender: string;
    burnUusd: ({ amount, minAmountOut, referrer, }: {
        amount: Uint128;
        minAmountOut?: OutputTokenMapForUint128;
        referrer?: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    registerStartingUser: ({ user, }: {
        user: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updateSlotSize: ({ slotSize, }: {
        slotSize: Uint128;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    createRound: ({ round, }: {
        round: SwapRound;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updateRound: ({ params, }: {
        params: UpdateRoundParams;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    deleteRound: ({ id, }: {
        id: number;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    createGuild: ({ name, referrer, slug, }: {
        name: string;
        referrer?: string;
        slug: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    migrateGuild: ({ guildId, referrer, }: {
        guildId: number;
        referrer?: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export declare class BurndropMsgComposer implements BurndropMsg {
    sender: string;
    contractAddress: string;
    constructor(sender: string, contractAddress: string);
    burnUusd: ({ amount, minAmountOut, referrer, }: {
        amount: Uint128;
        minAmountOut?: OutputTokenMapForUint128 | undefined;
        referrer?: string | undefined;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    registerStartingUser: ({ user, }: {
        user: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updateSlotSize: ({ slotSize, }: {
        slotSize: Uint128;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    createRound: ({ round, }: {
        round: SwapRound;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updateRound: ({ params, }: {
        params: UpdateRoundParams;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    deleteRound: ({ id, }: {
        id: number;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    createGuild: ({ name, referrer, slug, }: {
        name: string;
        referrer?: string | undefined;
        slug: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    migrateGuild: ({ guildId, referrer, }: {
        guildId: number;
        referrer?: string | undefined;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
//# sourceMappingURL=Burndrop.message-composer.d.ts.map