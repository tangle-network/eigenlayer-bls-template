use {{project-name | snake_case}} as blueprint;
use blueprint::{TangleTaskManager, TASK_MANAGER_ADDRESS};
use blueprint_sdk::alloy::primitives::Address;
use blueprint_sdk::logging::info;
use blueprint_sdk::macros::main;
use blueprint_sdk::runners::core::runner::BlueprintRunner;
use blueprint_sdk::runners::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::utils::evm::get_provider_http;

#[main(env)]
async fn main() {
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

    info!("Starting the event watcher ...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::new(eigen_config, env)
        .job(say_hello_job)
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
