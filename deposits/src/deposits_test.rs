#[cfg(not(feature = "library"))]

// use crate::error::ContractError;
use crate::msg::{BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use cw20::{Cw20ReceiveMsg, Cw20ExecuteMsg};

use cosmwasm_std::{Binary, Uint128, to_binary, CosmosMsg, SubMsg, WasmMsg};
use crate::deposits::query;
use crate::deposits::execute;
use crate::deposits::instantiate;

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    };
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator",  &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {
            token:"token".to_string(), 
            address:"tester".to_string()
        }).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Uint128::from(0u128), value.balance);
    }

    #[test]
    fn deposit() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let deposit = Uint128::new(7890);
        // beneficiary can release it
        let info = mock_info("token", &[]);
        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from("tester"),
            amount: deposit,
            msg: Binary::from_base64("test").unwrap(),
        });
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {
            token:"token".to_string(), 
            address:"tester".to_string()
        }).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(deposit, value.balance);
    }

    #[test]
    fn withdraw() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let deposit = Uint128::new(7890);
        // beneficiary can release it
        let info = mock_info("token", &[]);
        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from("tester"),
            amount: deposit,
            msg: Binary::from_base64("test").unwrap(),
        });
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {
            token:"token".to_string(), 
            address:"tester".to_string()
        }).unwrap();
        let old_value: BalanceResponse = from_binary(&res).unwrap();

        let withdrawal = Uint128::new(1000);

        let info = mock_info("tester", &[]);
        let msg = ExecuteMsg::Withdraw{
            token: "token".to_string(),
            amount: withdrawal
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {
            token:"token".to_string(), 
            address:"tester".to_string()
        }).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();

        assert_eq!(old_value.balance-withdrawal, value.balance);

        let binary_msg = to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: "tester".to_string(),
            amount: withdrawal,
        })
        .unwrap();
        // and this is how it must be wrapped for the vm to process it
        assert_eq!(
            _res.messages[0],
            SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "token".to_string(),
                msg: binary_msg,
                funds: vec![],
            }))
        );
    }
}
