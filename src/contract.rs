#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, SubMsg,
    WasmMsg, WasmQuery,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    CntExecuteMsg, CntQueryMsg, ExecuteMsg, InstantiateMsg, QueryMsg, QueryMsgResponse,
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
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetWithMsg {
            contract_addr,
            count,
        } => set_with_msg(deps, info, contract_addr, count),
        ExecuteMsg::SetWithSubMsg {
            contract_addr,
            count,
        } => set_with_sub_msg(deps, info, contract_addr, count),
    }
}

pub fn set_with_msg(
    _deps: DepsMut,
    _info: MessageInfo,
    contract_addr: String,
    count: u8,
) -> Result<Response, ContractError> {
    let cnt_msg = CntExecuteMsg::Set { count };

    let new_msg = WasmMsg::Execute {
        contract_addr,
        msg: to_binary(&cnt_msg)?,
        funds: Vec::<Coin>::new(),
    };

    Ok(Response::new()
        .add_message(new_msg)
        .add_attribute("method", "set_with_msg")
        .add_attribute("expected_count", count.to_string()))
}

pub fn set_with_sub_msg(
    _deps: DepsMut,
    _info: MessageInfo,
    contract_addr: String,
    count: u8,
) -> Result<Response, ContractError> {
    let cnt_msg = CntExecuteMsg::Set { count };

    let new_msg = WasmMsg::Execute {
        contract_addr,
        msg: to_binary(&cnt_msg)?,
        funds: Vec::<Coin>::new(),
    };

    const REPLY_ID: u64 = 1;
    let sub_msg = SubMsg::<Empty>::reply_on_success(new_msg, REPLY_ID);

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("method", "set_with_sub_msg")
        .add_attribute("expected_count", count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryWithWasmQuery { contract_addr } => query_with_wasm_query(contract_addr),
    }
}

pub fn query_with_wasm_query(contract_addr: String) -> StdResult<Binary> {
    let cnt_query_msg = CntQueryMsg::GetCount {};

    let wasm_query = WasmQuery::Smart {
        contract_addr,
        msg: to_binary(&cnt_query_msg)?,
    };

    to_binary(&QueryMsgResponse { data: wasm_query })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{CntQueryMsg, ExecuteMsg, InstantiateMsg, QueryMsg, QueryMsgResponse};
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{
        attr, from_binary, to_binary, Empty, Env, MessageInfo, OwnedDeps, Response, WasmQuery,
    };

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
        let msg = ExecuteMsg::SetWithMsg {
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
    fn test_set_with_sub_msg() {
        const COUNT: u8 = 222;
        let (mut deps, env, info, _) = get_instance(ALICE_ADDR);
        let msg = ExecuteMsg::SetWithSubMsg {
            contract_addr: CONTRACT_ADDR.to_string(),
            count: COUNT,
        };
        let res = execute(deps.as_mut(), env, info, msg);

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "set_with_sub_msg"),
                attr("expected_count", COUNT.to_string())
            ]
        )
    }

    #[test]
    fn test_query() {
        let (deps, env, _, _) = get_instance(ALICE_ADDR);
        let msg = QueryMsg::QueryWithWasmQuery {
            contract_addr: CONTRACT_ADDR.to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res = from_binary::<QueryMsgResponse>(&bin).unwrap();

        assert_eq!(
            res.data,
            WasmQuery::Smart {
                contract_addr: CONTRACT_ADDR.to_string(),
                msg: to_binary(&CntQueryMsg::GetCount {}).unwrap(),
            }
        );
    }
}
