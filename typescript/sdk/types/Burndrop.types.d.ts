/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */
export type Uint128 = string;
export interface InstantiateMsg {
  default_query_limit: number;
  genesis_guild_name: string;
  initial_slot_size: Uint128;
  max_query_limit: number;
  rounds: SwapRound[];
}
export interface SwapRound {
  ancs_liquidity: LiquidityPair;
  ancs_weight: number;
  end_time: number;
  id: number;
  oppamint_liquidity: LiquidityPair;
  oppamint_weight: number;
  start_time: number;
  [k: string]: unknown;
}
export interface LiquidityPair {
  x: Uint128;
  y: Uint128;
  [k: string]: unknown;
}
export type ExecuteMsg =
  | {
      burn_uusd: {
        amount: Uint128;
        min_amount_out?: OutputTokenMapForUint128 | null;
        referrer?: string | null;
      };
    }
  | {
      register_starting_user: {
        user: string;
      };
    }
  | {
      update_slot_size: {
        slot_size: Uint128;
      };
    }
  | {
      create_round: {
        round: SwapRound;
      };
    }
  | {
      update_round: {
        params: UpdateRoundParams;
      };
    }
  | {
      delete_round: {
        id: number;
      };
    }
  | {
      create_guild: {
        name: string;
        referrer?: string | null;
      };
    }
  | {
      migrate_guild: {
        guild_id: number;
        referrer?: string | null;
      };
    };
export interface OutputTokenMapForUint128 {
  ancs: Uint128;
  oppamint: Uint128;
  [k: string]: unknown;
}
export interface UpdateRoundParams {
  ancs_liquidity?: LiquidityPair | null;
  end_time?: number | null;
  id: number;
  oppamint_liquidity?: LiquidityPair | null;
  start_time?: number | null;
  [k: string]: unknown;
}
export type QueryMsg =
  | {
      config: {};
    }
  | {
      user_info: {
        address: string;
      };
    }
  | {
      users_info: {
        limit?: number | null;
        order?: OrderBy | null;
        start?: string | null;
      };
    }
  | {
      current_price: {};
    }
  | {
      simulate_burn: {
        amount: Uint128;
      };
    }
  | {
      rounds: {};
    }
  | {
      guild_info: {
        guild_id: number;
      };
    };
export type OrderBy = 'ascending' | 'descending';
export interface MigrateMsg {}
export type Addr = string;
export interface Config {
  default_query_limit: number;
  max_query_limit: number;
  owner: Addr;
  slot_size: Uint128;
  [k: string]: unknown;
}
export type Decimal = string;
export interface PriceResponse {
  price: OutputTokenMapForDecimal;
}
export interface OutputTokenMapForDecimal {
  ancs: Decimal;
  oppamint: Decimal;
  [k: string]: unknown;
}
export interface GuildInfoResponse {
  burned_uusd: Uint128;
}
export interface RoundsResponse {
  rounds: SwapRound[];
}
export interface SimulateBurnResponse {
  final_amount: Uint128;
  swapped_out: OutputTokenMapForUint128;
}
export interface UserInfoResponse {
  burnable: Uint128;
  burned: Uint128;
  cap: Uint128;
  guild_contributed_uusd: Uint128;
  guild_id: number;
  slot_size: Uint128;
  slots: Uint128;
  swapped_out: OutputTokenMapForUint128;
}
export interface UsersInfoResponse {
  users: [string, UserInfoResponse][];
}
//# sourceMappingURL=Burndrop.types.d.ts.map
