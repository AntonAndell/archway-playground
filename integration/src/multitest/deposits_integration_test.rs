use cosmwasm_std::{Uint128, Addr, Binary};
use cw_multi_test::{App, ContractWrapper};

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
fn deposit() {
    // Arrange
    let owner = Addr::unchecked("owner");
    let mut router = App::default();

    let deposits_contract = crate::contracts::deposits_contract::build(owner.clone(), &mut router);
    let initial_balance = Uint128::new(7890);
    let token_contract = crate::contracts::token_contract::build(owner.clone(), &mut router, initial_balance);

    let balance = token_contract.get_balance(&mut router, owner.clone().into());
    assert_eq!(balance.balance, initial_balance);

    // Act
    let _ = token_contract.send(
        owner.clone(), 
        &mut router, 
        deposits_contract.addr().clone().into(),
        Uint128::new(100),
        Binary::from(r#"{"some":123}"#.as_bytes())
    );

    // Assert
    let deposit_balance = deposits_contract.get_balance(&mut router, token_contract.addr().clone().into(), owner.clone().into());

    assert_eq!(deposit_balance.balance,  Uint128::new(100));
}
