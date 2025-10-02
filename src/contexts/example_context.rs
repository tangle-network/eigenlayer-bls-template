use crate::contexts::client::AggregatorClient;
use blueprint_sdk::macros::context::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;

// TODO: Replace with your context name
#[derive(Clone, KeystoreContext)]
pub struct ExampleContext {
  pub client: AggregatorClient,
  #[config]
  pub std_config: BlueprintEnvironment,
}