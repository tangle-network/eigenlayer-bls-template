use {{project-name | snake_case}} as blueprint;


use blueprint_sdk::alloy::network::EthereumWallet;
use blueprint_sdk::alloy::primitives::{address, Address, Bytes};
use blueprint_sdk::alloy::signers::local::PrivateKeySigner;
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::{info, warn, tokio, Router};
use std::time::Duration;
use blueprint::TangleTaskManager;
use blueprint::{PRIVATE_KEY, TASK_MANAGER_ADDRESS};
// TODO: Replace with your context name
use blueprint::ExampleContext;
use blueprint::{EXAMPLE_JOB_ID, example_task};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    let env = BlueprintEnvironment::load()?;

    // TODO: Replace with your context name
    let context = ExampleContext {
        std_config: env.clone(),
    };

    let signer: PrivateKeySigner = PRIVATE_KEY.parse().expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());
    // Create an instance of your task manager
    let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider);

    // Spawn a task to create a task - this is just for testing/example purposes
    info!("Spawning a task to create a task on the contract...");
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            // We use the Anvil Account #4 as the Task generator address
            let task = contract
                .createNewTask(Bytes::from_static(b"World"), 100u32, vec![0].into())
                .from(address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65"));
            let receipt = task.send().await.unwrap().get_receipt().await.unwrap();
            if receipt.status() {
                info!("Task created successfully");
            } else {
                warn!("Task creation failed");
            }
        }
    });

    info!("Starting the event watcher ...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::builder(eigen_config, env)
        .router(
            // TODO: Update your task
            Router::new()
                .route(EXAMPLE_JOB_ID, example_task)
                .with_context(context),
        )
        .with_shutdown_handler(async {
            info!("Shutting down task manager service");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
