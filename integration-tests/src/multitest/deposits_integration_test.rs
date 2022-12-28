use cosmwasm_std::testing::{mock_env, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{Uint128, Addr, Binary};
use cw_multi_test::{App, Executor, BankKeeper, Contract, ContractWrapper};
use cw20::{Cw20Coin};

pub fn store_deposits_code(router: &mut App) -> u64 {
    use deposits::deposits::{execute, instantiate, query};

    let contract = ContractWrapper::new(execute, instantiate, query);
    router.store_code(Box::new(contract))
}

pub fn store_token_code(router: &mut App) -> u64 {
    use token::token::{execute, instantiate, query};

    let contract = ContractWrapper::new(execute, instantiate, query);
    router.store_code(Box::new(contract))
}

#[test]
fn query_value() {
    let owner = Addr::unchecked("owner");
    let mut router = App::default();

    let deposits_contract_code_id = store_deposits_code(&mut router);
    let token_contract_code_id = store_token_code(&mut router);

    // Setup the counter contract with an initial count of zero
    let init_msg_deposits = deposits::msg::InstantiateMsg {
    };    
    let initial_balance = Uint128::new(7890);
    let init_msg_token = token::msg::InstantiateMsg {
        name: "bnUSD".to_string(),
        symbol: "bnUSD".to_string(),
        decimals: 18,
        initial_balances: vec![Cw20Coin {
            address: owner.clone().into(),
            amount: initial_balance.clone(),
        }],
        mint: None,
        marketing: None
    };

    // Instantiate the counter contract using its newly stored code id
    let deposits_addr = router
        .instantiate_contract(deposits_contract_code_id, owner.clone(), &init_msg_deposits, &[], "deposits", None)
        .unwrap();


    let token_addr = router
        .instantiate_contract(token_contract_code_id, owner.clone(), &init_msg_token, &[], "deposits", None)
        .unwrap();

    let balance_msg =  token::msg::QueryMsg::Balance { 
        address: "owner".to_string()
    };
    let balance_response: cw20::BalanceResponse = router
        .wrap()
        .query_wasm_smart(token_addr.clone(), &balance_msg)
        .unwrap();

    assert_eq!(balance_response.balance, initial_balance);


    let deposit_msg = cw20::Cw20ExecuteMsg::Send {
        contract: deposits_addr.clone().into(),
        amount: Uint128::new(100),
        msg: Binary::from(r#"{"some":123}"#.as_bytes()),
    };
    let _ = router.execute_contract(
            owner.clone(),
            token_addr.clone(),
            &deposit_msg,
            &[],
        )
        .unwrap();

    let get_deposit =  deposits::msg::QueryMsg::GetBalance { 
        token: token_addr.clone().into(),
        address: "owner".to_string()
    };
    let deposit_balance: deposits::msg::BalanceResponse = router
        .wrap()
        .query_wasm_smart(deposits_addr.clone(), &get_deposit)
        .unwrap();

    assert_eq!(deposit_balance.balance,  Uint128::new(100));
}
