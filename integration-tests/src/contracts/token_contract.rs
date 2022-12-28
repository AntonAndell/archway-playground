use cosmwasm_std::{Uint128, Addr, Binary};
use cw_multi_test::{App, Executor, ContractWrapper, AppResponse};

use token::token::{execute, instantiate, query};
use cw20::{Cw20Coin, BalanceResponse};

pub fn build(sender: Addr, router: &mut App, initial_balance : Uint128) -> TokenContract {
    let contract = ContractWrapper::new(execute, instantiate, query);
    let id = router.store_code(Box::new(contract));
    let init_msg_token = token::msg::InstantiateMsg {
        name: "bnUSD".to_string(),
        symbol: "bnUSD".to_string(),
        decimals: 18,
        initial_balances: vec![Cw20Coin {
            address: sender.clone().into(),
            amount: initial_balance,
        }],
        mint: None,
        marketing: None
    };   

    let addr = router
        .instantiate_contract(id, sender, &init_msg_token, &[], "bnUSD", None)
        .unwrap();
    TokenContract{address:addr}
}

pub struct TokenContract{address: Addr}
impl TokenContract {
    pub fn addr(&self) -> &Addr {
        &self.address
    }

    pub fn get_balance(&self, router: &mut App,  address: String) ->  BalanceResponse {
        let balance_msg =  token::msg::QueryMsg::Balance { 
            address: address
        };
        router
            .wrap()
            .query_wasm_smart(self.address.clone(), &balance_msg)
            .unwrap()
    }


    pub fn send(&self, sender:Addr, router: &mut App, contract: String, amount:Uint128, msg: Binary) ->  AppResponse {
        let send_msg = cw20::Cw20ExecuteMsg::Send {
            contract: contract,
            amount: amount,
            msg: msg,
        };
        router.execute_contract(
            sender,
            self.addr().clone(),
            &send_msg,
            &[],
        )
        .unwrap()
    }

}
