use {{project-name | snake_case}} as blueprint;
use blueprint::{TangleTaskManager, TASK_MANAGER_ADDRESS};
use blueprint_sdk::alloy::primitives::{Address, U256};
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_provider_http;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::{Router, info, warn};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    // Load the blueprint environment
    let env = BlueprintEnvironment::load()?;

    // Create your service context
    // Here you can pass any configuration or context that your service needs.
    let context = blueprint::ExampleContext {
        config: env.clone(),
    };

    // Get the provider
    let rpc_endpoint = env.http_rpc_endpoint.clone();
    let provider = Arc::new(get_provider_http(&rpc_endpoint));

    // Create an instance of your task manager
    let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider.clone());

    // Create a polling producer to listen for contract events
    let task_producer = PollingProducer::new(
        provider.clone(),
        PollingConfig::default().poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    // Spawn a task to create a task - this is just for testing/example purposes
    info!("Spawning a task to create a task on the contract...");
    let rpc_endpoint_clone = rpc_endpoint.clone();
    blueprint_sdk::tokio::spawn(async move {
        let provider = get_provider_http(&rpc_endpoint_clone);
        let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider);
        loop {
            blueprint_sdk::tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            // We use the Anvil Account #4 as the Task generator address
            let task = contract
                .createNewTask(U256::from(5), 100u32, vec![0].into())
                .from(blueprint_sdk::alloy::primitives::address!(
                    "15d34AAf54267DB7D7c367839AAf71A00a2C6A65"
                ));
            match task.send().await {
                Ok(pending) => match pending.get_receipt().await {
                    Ok(receipt) => {
                        if receipt.status() {
                            info!("Task created successfully");
                        } else {
                            warn!("Task creation failed");
                        }
                    }
                    Err(e) => warn!("Failed to get receipt: {:?}", e),
                },
                Err(e) => warn!("Failed to send task: {:?}", e),
            }
        }
    });

    info!("Starting the event watcher ...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default())
        .with_exit_after_register(false);

    BlueprintRunner::builder(eigen_config, env)
        .router(
            Router::new()
                .always(blueprint::say_hello)
                .with_context(context),
        )
        .producer(task_producer)
        .with_shutdown_handler(async {
            blueprint_sdk::info!("Shutting down...");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
