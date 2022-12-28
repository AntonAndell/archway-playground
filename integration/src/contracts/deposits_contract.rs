use cosmwasm_std::{Addr};
use cw_multi_test::{App, Executor, ContractWrapper};

use deposits::deposits::{execute, instantiate, query};
use deposits::msg::BalanceResponse;

pub fn build(sender: Addr, router: &mut App) -> DepositsContract {
    let contract = ContractWrapper::new(execute, instantiate, query);
    let id = router.store_code(Box::new(contract));
    let init_msg_deposits = deposits::msg::InstantiateMsg {
    };    

    let addr = router
        .instantiate_contract(id, sender, &init_msg_deposits, &[], "deposits", None)
        .unwrap();


    DepositsContract{address: addr}
}

pub struct DepositsContract{address: Addr}
impl DepositsContract {
    pub fn addr(&self) -> &Addr {
        &self.address
    }

    pub fn get_balance(&self, router: &mut App, token: String, address: String) ->  BalanceResponse {
        let get_deposit =  deposits::msg::QueryMsg::GetBalance { 
            token: token,
            address: address
        };
        
        router
        .wrap()
        .query_wasm_smart(self.address.clone(), &get_deposit)
        .unwrap()
    
    }

}
