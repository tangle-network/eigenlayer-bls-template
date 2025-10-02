#![allow(dead_code)]
use crate::ITangleTaskManager::TaskResponse;
use crate::TangleTaskManager::NewTaskCreated;
use crate::contexts::client::SignedTaskResponse;
use crate::contexts::combined::CombinedContext;
use crate::error::TaskError;
use blueprint_sdk::alloy::primitives::{keccak256};
use blueprint_sdk::alloy::core::sol_types::{SolEvent, SolType, SolValue};
use blueprint_sdk::contexts::keystore::KeystoreContext;
use blueprint_sdk::crypto::bn254::ArkBlsBn254;
use blueprint_sdk::evm::extract::BlockEvents;
use blueprint_sdk::extract::Context;
use blueprint_sdk::keystore::backends::bn254::Bn254Backend;
use blueprint_sdk::keystore::backends::Backend;
use blueprint_sdk::{error, info};
use blueprint_sdk::eigensdk::crypto_bls::BlsKeyPair;
use blueprint_sdk::eigensdk::types::operator::operator_id_from_g1_pub_key;

// TODO: Replace with your job id identifier
pub const EXAMPLE_JOB_ID: u32 = 0;

/// TODO: Replace with your job logic
/// Sends a signed task response to the BLS Aggregator.
/// This job is triggered by the `NewTaskCreated` event emitted by the `TangleTaskManager`.
/// The job say hello and sends the signed task response to the BLS Aggregator.
#[blueprint_sdk::macros::debug_job]
pub async fn example_task(
    Context(ctx): Context<CombinedContext>,
    BlockEvents(events): BlockEvents,
) -> Result<(), TaskError> {
    let client = ctx.example_context.client.clone();

    let task_created_events = events.iter().filter_map(|log| {
        NewTaskCreated::decode_log(&log.inner)
            .map(|event| event.data)
            .ok()
    });

    // TODO: Replace with your use cases
    for task_created in task_created_events {
        let task = task_created.task;
        let task_index = task_created.taskIndex;

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

        // Calculate our response to job
        let task_response = TaskResponse {
            referenceTaskIndex: task_index,
            message: message.into(),
        };

        let bn254_public = ctx.keystore().first_local::<ArkBlsBn254>().unwrap();
        let bn254_secret = match ctx.keystore().expose_bls_bn254_secret(&bn254_public) {
            Ok(s) => match s {
                Some(s) => s,
                None => {
                    return Err(TaskError::Task(
                        "Failed to send signed task response".to_string(),
                    ));
                }
            },
            Err(e) => {
                return Err(TaskError::Task(format!(
                    "Failed to send signed task response: {e:?}",
                )));
            }
        };
        let bls_key_pair = match BlsKeyPair::new(bn254_secret.0.to_string()) {
            Ok(pair) => pair,
            Err(e) => {
                return Err(TaskError::Task(format!(
                    "Failed to send signed task response: {e:?}",
                )));
            }
        };
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key())?;

        // Sign the Hashed Message and send it to the BLS Aggregator
        let msg_hash = keccak256(<TaskResponse as SolType>::abi_encode(&task_response));
        let signed_response = SignedTaskResponse {
            task_response,
            signature: bls_key_pair.sign_message(msg_hash.as_ref()),
            operator_id,
        };

        info!(
            "Sending signed task response to BLS Aggregator: {:#?}",
            signed_response
        );
        if let Err(e) = client.send_signed_task_response(signed_response).await {
            error!("Failed to send signed task response: {e:?}");
            return Err(TaskError::Task(format!(
                "Failed to send signed task response: {e:?}",
            )));
        }
    }

    Ok(())
}