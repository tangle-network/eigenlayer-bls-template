use crate::contexts::aggregator::AggregatorContext;
// TODO: Replace with your context name
use crate::contexts::example_context::ExampleContext;
use blueprint_sdk::macros::context::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;

/// Combined context that includes both the ExampleContext and AggregatorContext
/// This allows both jobs to share the same context in the router
#[derive(Clone, KeystoreContext)]
pub struct CombinedContext {
    // TODO: Replace with your context name
    pub example_context: ExampleContext,
    pub aggregator_context: Option<AggregatorContext>,
    #[config]
    pub std_config: BlueprintEnvironment,
}

impl CombinedContext {
    pub fn new(
        // TODO: Replace with your context name
        example_context: ExampleContext,
        aggregator_context: Option<AggregatorContext>,
        std_config: BlueprintEnvironment,
    ) -> Self {
        Self {
            example_context,
            aggregator_context,
            std_config,
        }
    }
}