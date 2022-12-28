#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{WasmMsg, Uint128, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw20::{Cw20ReceiveMsg, Cw20ExecuteMsg};
use crate::error::ContractError;
use crate::msg::{BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BALANCES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:deposits";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw {token, amount} => withdraw(deps,_env, info, token, amount),
        ExecuteMsg::Receive(msg) => deposit(deps,_env, info, msg)
    }
}

pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount <= Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let token = deps.api.addr_validate(&token)?;

    BALANCES.update(
        deps.storage,
        (&token, &info.sender),
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;

    let msg = Cw20ExecuteMsg::Transfer {
        recipient: info.sender.to_string(),
        amount: amount,
    };
    let exec = WasmMsg::Execute {
        contract_addr:token.into(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };
    Ok(Response::new()
        .add_message(exec)
    )
}

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    if msg.amount <= Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let address = deps.api.addr_validate(&msg.sender)?;

    BALANCES.update(
        deps.storage,
        (&info.sender, &address),
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default() + msg.amount)
        },
    )?;

    return Ok(Response::new()
        .add_attribute("action", "refund"));
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {token, address} => to_binary(&query_balance(deps, token, address)?),
    }
}

pub fn query_balance(deps: Deps, token: String, address: String) -> StdResult<BalanceResponse> {
    let address = deps.api.addr_validate(&address)?;
    let token = deps.api.addr_validate(&token)?;
    let balance = BALANCES
        .may_load(deps.storage, (&token, &address))?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}