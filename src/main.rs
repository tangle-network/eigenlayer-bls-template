use alloy_primitives::Address;
use {{project-name | snake_case}} as blueprint;
use blueprint::{TangleTaskManager, TASK_MANAGER_ADDRESS};
use color_eyre::Result;
use gadget_sdk as sdk;
use gadget_sdk::utils::evm::get_provider_http;
use sdk::runners::eigenlayer::EigenlayerBLSConfig;
use sdk::runners::BlueprintRunner;

#[sdk::main(env)]
async fn main() -> Result<()> {
    // Create your service context
    // Here you can pass any configuration or context that your service needs.
    let context = blueprint::ExampleContext {
        config: env.clone(),
    };

    // Get the provider
    let provider = get_provider_http(&env.http_rpc_endpoint);

    // Create an instance of your task manager
    let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider);

    // Create the event handler from the job
    let say_hello_job = blueprint::SayHelloEventHandler::new(contract, context);

    tracing::info!("Starting the event watcher ...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::new(eigen_config, env)
        .job(say_hello_job)
        .run()
        .await?;

    tracing::info!("Exiting...");
    Ok(())
}
