#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetBetResponse, InstantiateMsg, QueryMsg};
use crate::state::{Bet, Config, BETS, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cosmos-gaming";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); // 0.1.0

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // This will error, if the user gives an invalid address, "foo"
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        admin_address: validated_admin_address, // Set to the validated address
    };

    CONFIG.save(deps.storage, &config)?;

    // Result<Response>
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateBet { teams } => execute_create_bet(deps, env, info, teams),
        ExecuteMsg::PlaceBet { teams, choice } => execute_bet(deps, env, info, teams, choice),
    }
}

fn execute_create_bet(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    teams: String,
) -> Result<Response, ContractError> {
    // Does the map have a key of this value
    if BETS.has(deps.storage, teams.clone()) {
        // If it does, we want to error!
        return Err(ContractError::CustomError {
            val: "Key already taken!".to_string(),
        });
    }

    let bet = Bet {
        teams: teams.clone(),
        team_a: 0,
        team_b: 0,
    };

    BETS.save(deps.storage, teams, &bet)?;

    Ok(Response::new().add_attribute("action", "create_bet"))
}

fn execute_bet(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    teams: String,
    choice: String,
) -> Result<Response, ContractError> {
    // If there is no bet with the key teams
    if !BETS.has(deps.storage, teams.clone()) {
        // We want to error and tell the user that bet does not exist
        return Err(ContractError::CustomError {
            val: "Bet does not exist!".to_string(),
        });
    }

    let mut bet = BETS.load(deps.storage, teams.clone())?;

    // If choice is not yes or no
    if choice != "yes" && choice != "no" {
        Err(ContractError::CustomError {
            val: "Unrecognised choice!".to_string(),
        })
    } else {
        // If its yes add to the yes bets
        // If its no add to the no bets
        if choice == "yes" {
            bet.team_a += 1;
        } else {
            bet.team_b += 1;
        }

        // Save the updated bet to the chain
        BETS.save(deps.storage, teams, &bet)?;
        Ok(Response::new().add_attribute("action", "bet"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTeams { teams } => query_get_bets(deps, env, teams),
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(deps.storage)?),
    }
}

fn query_get_bets(deps: Deps, _env: Env, teams: String) -> StdResult<Binary> {
    let bet = BETS.may_load(deps.storage, teams)?;
    to_binary(&GetBetResponse { bet })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, GetBetResponse, InstantiateMsg, QueryMsg};
    use crate::state::{Bet, Config};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(), // String, String::from("addr1")
        };

        let resp = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "instantiate")]);

        let msg = QueryMsg::GetConfig {};
        let resp = query(deps.as_ref(), env, msg).unwrap();
        let config: Config = from_binary(&resp).unwrap();
        assert_eq!(
            config,
            Config {
                admin_address: Addr::unchecked("addr1")
            }
        );
    }

    #[test]
    fn test_create_bet() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(), // String, String::from("addr1")
        };

        // Before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreateBet {
            teams: "FC Barcelona Vs Manchester City".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "create_bet")]);

        let msg = QueryMsg::GetTeams {
            teams: "FC Barcelona Vs Manchester City".to_string(),
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_bet_response: GetBetResponse = from_binary(&resp).unwrap();
        assert_eq!(
            get_bet_response,
            GetBetResponse {
                bet: Some(Bet {
                    teams: "FC Barcelona Vs Manchester City".to_string(),
                    team_a: 0,
                    team_b: 0
                })
            }
        );

        let msg = ExecuteMsg::CreateBet {
            teams: "FC Barcelona Vs Manchester City".to_string(),
        };
        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn test_bet() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(), // String, String::from("addr1")
        };

        // Before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // We need a bet to bet on!
        let msg = ExecuteMsg::CreateBet {
            teams: "FC Barcelona Vs Manchester City".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Success case, we bet on a bet that exists, with a valid option
        let msg = ExecuteMsg::PlaceBet {
            teams: "FC Barcelona Vs Manchester City".to_string(),
            choice: "yes".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "bet"),]);

        let msg = QueryMsg::GetTeams {
            teams: "FC Barcelona Vs Manchester City".to_string(),
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_bet_response: GetBetResponse = from_binary(&resp).unwrap();
        assert_eq!(
            get_bet_response,
            GetBetResponse {
                bet: Some(Bet {
                    teams: "FC Barcelona Vs Manchester City".to_string(),
                    team_a: 1,
                    team_b: 0
                })
            }
        );

        // Error case 1: we bet on a bet that does not exist
        let msg = ExecuteMsg::PlaceBet {
            teams: "".to_string(),
            choice: "no".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Error case 2: we bet on a bet that exists, but with an invalid choice
        let msg = ExecuteMsg::PlaceBet {
            teams: "FC Barcelona Vs Manchester City".to_string(),
            choice: "maybe".to_string(),
        };
        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
}
