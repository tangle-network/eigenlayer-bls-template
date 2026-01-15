use blueprint_sdk::alloy::primitives::{address, Address};
use blueprint_sdk::alloy::rpc::types::Log;
use blueprint_sdk::alloy::sol;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::macros::load_abi;
use std::convert::Infallible;
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    TangleTaskManager,
    "contracts/out/TangleTaskManager.sol/TangleTaskManager.json"
);

load_abi!(
    TANGLE_TASK_MANAGER_ABI_STRING,
    "contracts/out/TangleTaskManager.sol/TangleTaskManager.json"
);

pub static TASK_MANAGER_ADDRESS: LazyLock<Address> = LazyLock::new(|| {
    std::env::var("TASK_MANAGER_ADDRESS")
        .map(|addr| addr.parse().expect("Invalid TASK_MANAGER_ADDRESS"))
        .unwrap_or_else(|_| address!("0000000000000000000000000000000000000000"))
});

#[derive(Clone)]
pub struct ExampleContext {
    pub config: BlueprintEnvironment,
}

/// Returns "Hello, {who}!"
pub async fn say_hello(context: ExampleContext, who: String) -> Result<String, Infallible> {
    blueprint_sdk::info!("Successfully ran job function!");
    println!("Successfully ran job function!");
    Ok(format!("Hello, {who}!"))
}

/// Example pre-processor for handling inbound events
pub async fn example_pre_processor(
    (_event, log): (TangleTaskManager::NewTaskCreated, Log),
) -> Option<(String,)> {
    let who = log.address();
    Some((who.to_string(),))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = BlueprintEnvironment::default();
        let context = ExampleContext { config };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(say_hello(context, "Alice".into())).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }
}
