use crate::sdk::load_abi;
use alloy_sol_types::sol;
use gadget_sdk as sdk;
use serde::{Deserialize, Serialize};

use alloy_primitives::{address, Address};
use gadget_sdk::event_listener::evm::contracts::EvmContractEventListener;
use std::{convert::Infallible, ops::Deref};
use structopt::lazy_static::lazy_static;

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

lazy_static! {
    pub static ref TASK_MANAGER_ADDRESS: Address = std::env::var("TASK_MANAGER_ADDRESS")
        .map(|addr| addr.parse().expect("Invalid TASK_MANAGER_ADDRESS"))
        .unwrap_or_else(|_| address!("0000000000000000000000000000000000000000"));
}

#[derive(Clone)]
pub struct ExampleContext {
    pub config: sdk::config::StdGadgetConfiguration,
}

/// Returns "Hello, {who}!"
#[sdk::job(
    id = 0,
    params(who),
    event_listener(
        listener = EvmContractEventListener<TangleTaskManager::NewTaskCreated>,
        instance = TangleTaskManager,
        abi = TANGLE_TASK_MANAGER_ABI_STRING,
        pre_processor = example_pre_processor,
    ),
)]
pub fn say_hello(context: ExampleContext, who: String) -> Result<String, Infallible> {
    Ok(format!("Hello, {who}!"))
}

/// Example pre-processor for handling inbound events
async fn example_pre_processor(
    (_event, log): (TangleTaskManager::NewTaskCreated, alloy_rpc_types::Log),
) -> Result<(String,), gadget_sdk::Error> {
    let who = log.address();
    Ok((who.to_string(),))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = sdk::config::StdGadgetConfiguration::default();
        let context = ExampleContext { config };
        let result = say_hello(context, "Alice".into()).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }
}
