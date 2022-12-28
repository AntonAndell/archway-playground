pub mod deposits;

mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

#[cfg(test)]
mod deposits_test;