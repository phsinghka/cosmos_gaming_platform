use crate::state::Bet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// How do we communicate with our contract

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin_address: String, // Why is this String not Addr? So we can validate it!
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateBet {
        // ExecuteMsg::CreateBet { teams: "Do you love Spark IBC?" }
        teams: String,
    },
    PlaceBet {
        teams: String,  // what teams are we responding too?
        choice: String, // what is our answer? "yes" or "no"
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetTeams { teams: String },
    GetConfig {},
}

// This is what we return for our GetTeams route
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetBetResponse {
    pub bet: Option<Bet>, // Option means it can either be null (None) or a Bet
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
