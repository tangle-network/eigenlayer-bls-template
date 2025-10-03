pub mod error;

use crate::error::TaskError;
use crate::TangleTaskManager::NewTaskCreated;
use blueprint_sdk::alloy::primitives::{address, Address};
use blueprint_sdk::alloy::sol_types::{SolEvent, SolValue};
use blueprint_sdk::alloy::sol;
use blueprint_sdk::info;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::extract::Context;
use blueprint_sdk::evm::extract::BlockEvents;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;

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

// TODO: Replace with your context name
#[derive(Clone)]
pub struct ExampleContext {
    pub std_config: BlueprintEnvironment,
}

pub const EXAMPLE_JOB_ID: u32 = 0;

/// Example task that responds to a task created event
/// This function is triggered by the NewTaskCreated event emitted by the TangleTaskManager contract
/// This function response to greeting `Task.message`
#[blueprint_sdk::macros::debug_job]
pub async fn example_task(
    Context(_ctx): Context<ExampleContext>,
    BlockEvents(events): BlockEvents,
) -> Result<(), TaskError> {
    info!("Successfully ran job function!");

    let task_created_events = events.iter().filter_map(|log| {
        NewTaskCreated::decode_log(&log.inner)
            .map(|event| event.data)
            .ok()
    });

    for task_created in task_created_events {
        let task = task_created.task;
        let task_index = task_created.taskIndex;

        info!("Task created: {}", task_index);

        let message_bytes = &task.message;
        let greeting = std::str::from_utf8(message_bytes)
            .unwrap_or("<invalid utf8>")
            .to_string();
        info!("Greeting: {}", greeting);

        // Calculate the square
        let greeting_result = format!("Hello, {}!", greeting);
        info!("Greeting result: {}", greeting_result);

        // Properly encode the result as a uint256 instead of a string
        let message = SolValue::abi_encode(&greeting_result.as_bytes());
        info!("Result message: {:?}", message);
    }

    Ok(())
}
