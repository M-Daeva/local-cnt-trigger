#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmMsg, WasmQuery,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteCntMsg, ExecuteTrgMsg, InstantiateMsg, QueryCntMsg, QueryCntResponse, QueryTrgMsg,
    QueryTrgResponse,
};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:local-cnt-trigger";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let initial_state = State { owner: info.sender };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STATE.save(deps.storage, &initial_state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", initial_state.owner.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteTrgMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteTrgMsg::SetWithMsg {
            contract_addr,
            count,
        } => set_with_msg(deps, info, contract_addr, count),
    }
}

pub fn set_with_msg(
    _deps: DepsMut,
    _info: MessageInfo,
    contract_addr: String,
    count: u8,
) -> Result<Response, ContractError> {
    let wasm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr,
        msg: to_binary(&ExecuteCntMsg::Set { count })?,
        funds: Vec::<Coin>::new(),
    });

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("method", "set_with_msg")
        .add_attribute("expected_count", count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryTrgMsg) -> StdResult<Binary> {
    match msg {
        QueryTrgMsg::SmartQuery { contract_addr } => wasm_query(deps, contract_addr),
    }
}

pub fn wasm_query(deps: Deps, contract_addr: String) -> StdResult<Binary> {
    let res: QueryCntResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr,
        msg: to_binary(&QueryCntMsg::GetCount {})?,
    }))?;

    to_binary(&QueryTrgResponse {
        expected_count: res.count,
    })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteTrgMsg, InstantiateMsg, QueryTrgMsg, QueryTrgResponse};
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{attr, from_binary, Empty, Env, MessageInfo, OwnedDeps, Response};

    pub const CONTRACT_ADDR: &str = "juno1gjqnuhv52pd2a7ets2vhw9w9qa9knyhyqd4qeg";
    pub const ALICE_ADDR: &str = "juno1chgwz55h9kepjq0fkj5supl2ta3nwu638camkg";

    type Instance = (
        OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        Env,
        MessageInfo,
        Result<Response, ContractError>,
    );

    fn get_instance(addr: &str) -> Instance {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr, &[]);
        let msg = InstantiateMsg {};

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
        (deps, env, info, res)
    }

    #[test]
    fn test_init() {
        let (_, _, _, res) = get_instance(ALICE_ADDR);

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "instantiate"),
                attr("owner", ALICE_ADDR.to_string()),
            ]
        )
    }

    #[test]
    fn test_set_with_msg() {
        const COUNT: u8 = 111;
        let (mut deps, env, info, _) = get_instance(ALICE_ADDR);
        let msg = ExecuteTrgMsg::SetWithMsg {
            contract_addr: CONTRACT_ADDR.to_string(),
            count: COUNT,
        };
        let res = execute(deps.as_mut(), env, info, msg);

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "set_with_msg"),
                attr("expected_count", COUNT.to_string())
            ]
        )
    }

    #[test]
    fn test_query() {
        let (deps, env, _, _) = get_instance(ALICE_ADDR);
        let msg = QueryTrgMsg::SmartQuery {
            contract_addr: CONTRACT_ADDR.to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: QueryTrgResponse = from_binary(&bin).unwrap();

        assert_eq!(res.expected_count, 42);
    }
}
