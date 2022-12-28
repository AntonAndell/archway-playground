
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;

                    //  (Token, User)
pub const BALANCES: Map<(&Addr, &Addr), Uint128> = Map::new("balances");