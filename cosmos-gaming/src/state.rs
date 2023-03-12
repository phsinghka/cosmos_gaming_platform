use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr, // juno1xyz
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bet {
    pub teams: String,
    pub team_a: u64,
    pub team_b: u64,
}

pub const CONFIG: Item<Config> = Item::new("config"); // This is stored on chain!

// String -> Bet
// "Do you love Spark IBC?" -> Bet {
//                              teams: "FC Barcelona Vs Manchester City",
//                              team_a: 100,
//                              team_b: 50
//                             }
pub const BETS: Map<String, Bet> = Map::new("polls"); // Stores bet variables, with a string index
