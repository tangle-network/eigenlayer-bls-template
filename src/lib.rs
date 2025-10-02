pub mod contexts;
pub mod jobs;
pub mod error;

use blueprint_sdk::alloy::primitives::{address, Address};
use blueprint_sdk::alloy::sol;
use std::env;
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    TangleTaskManager,
    "contracts/out/TangleTaskManager.sol/TangleTaskManager.json"
);

pub static TASK_MANAGER_ADDRESS: LazyLock<Address> = LazyLock::new(|| {
    env::var("TASK_MANAGER_ADDRESS")
        .map(|addr| addr.parse().expect("Invalid TASK_MANAGER_ADDRESS"))
        .unwrap_or_else(|_| address!("0000000000000000000000000000000000000000"))
});

pub static PRIVATE_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string()
    })
});
pub static AGGREGATOR_PRIVATE_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".to_string()
    })
});
