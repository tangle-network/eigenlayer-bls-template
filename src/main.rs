use {{project-name | snake_case}} as blueprint;

use blueprint_sdk::alloy::network::EthereumWallet;
use blueprint_sdk::alloy::primitives::{address, Address, Bytes};
use blueprint_sdk::alloy::signers::local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::{info, warn, tokio, Router};
use std::sync::Arc;
use std::time::Duration;
use blueprint::TangleTaskManager;
use blueprint::{PRIVATE_KEY, TASK_MANAGER_ADDRESS};
use blueprint::ExampleContext;
use blueprint::example_task;

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    tracing_subscriber::fmt::init();
    let env = BlueprintEnvironment::load()?;

    let context = ExampleContext {
        env: env.clone(),
    };

    let signer: PrivateKeySigner = PRIVATE_KEY.parse().expect("failed to generate wallet");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());
    let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider.clone());

    let task_producer = PollingProducer::new(
        Arc::new(provider),
        PollingConfig::from_current().step(1).confirmations(1u64).poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    info!("Spawning a task to create a task on the contract...");
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let task = contract
                .createNewTask(Bytes::from_static(b"World"), 100u32, vec![0].into())
                .from(address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65"));
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
                .always(example_task)
                .with_context(context),
        )
        .producer(task_producer)
        .with_shutdown_handler(async {
            info!("Shutting down task manager service");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
