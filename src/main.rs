use test_eigen_bls_blueprint as blueprint;

use blueprint::{AGGREGATOR_PRIVATE_KEY, TASK_MANAGER_ADDRESS};
use std::sync::Arc;
use std::time::Duration;
use blueprint_sdk::alloy::network::EthereumWallet;
use blueprint_sdk::alloy::primitives::Address;
use blueprint_sdk::alloy::signers::local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::{Router, info, tokio};

use blueprint::contexts::aggregator::AggregatorContext;
use blueprint::contexts::client::AggregatorClient;
use blueprint::contexts::combined::CombinedContext;
use blueprint::jobs::initialize_task::{initialize_bls_task, INITIALIZE_TASK_JOB_ID};
// TODO: Replace with your context name
use blueprint::contexts::example_context::ExampleContext;
use blueprint::jobs::example_task::{example_task, EXAMPLE_JOB_ID};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    let env = BlueprintEnvironment::load()?;

    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());
    let server_address = format!("{}:{}", "127.0.0.1", 8081);

    // TODO: Replace with your context name
    let context = ExampleContext {
        client: AggregatorClient::new(&server_address)
            .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?,
        std_config: env.clone(),
    };

    // Create the aggregator context
    let aggregator_context = AggregatorContext::new(
        server_address,
        *TASK_MANAGER_ADDRESS,
        wallet.clone(),
        env.clone(),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;


    // Create the combined context for both tasks
    let combined_context = CombinedContext::new(
        context,
        Some(aggregator_context.clone()),
        env.clone(),
    );
    let client = Arc::new(provider);

     // Create producer for task events
     let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig::default().poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    info!("Spawning a task to create a task on the contract...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::builder(eigen_config, BlueprintEnvironment::default())
    .router(
        Router::new()
            .route(EXAMPLE_JOB_ID, example_task)
            .route(INITIALIZE_TASK_JOB_ID, initialize_bls_task)
            .with_context(combined_context),
    )
    .producer(task_producer)
    .background_service(aggregator_context)
    .with_shutdown_handler(async {
        blueprint_sdk::info!("Shutting down task manager service");
    })
    .run()
    .await?;

    info!("Exiting...");
    Ok(())
}
