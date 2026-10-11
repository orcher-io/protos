//! The kernel protocol's messages are generated and round-trip as protobuf.

use orcher_proto::kernel::v1 as kernel;
use orcher_proto::prost::Message;

/// An activation with a resolution and a result with a satisfied and a new
/// command survive encoding and decoding unchanged.
#[test]
fn an_activation_and_its_result_round_trip() {
    let activation = kernel::RunActivation {
        activation_id: b"act-1".to_vec(),
        run: Some(kernel::RunRef {
            workflow_id: "order-7".into(),
            run_id: "run-1".into(),
            workflow_type: "checkout".into(),
            task_queue: "orders".into(),
            namespace: "default".into(),
        }),
        resolutions: vec![kernel::Resolution {
            step_id: "task_0".into(),
            at: None,
            replaying: true,
            what: Some(kernel::resolution::What::Step(kernel::StepSettled {
                outcome: Some(kernel::Outcome {
                    result: Some(kernel::outcome::Result::Value(kernel::Payload {
                        data: b"42".to_vec(),
                        metadata: Default::default(),
                    })),
                    end: kernel::End::Completed as i32,
                    attempts: 1,
                }),
            })),
        }],
        queries: vec![],
    };
    let decoded = kernel::RunActivation::decode(activation.encode_to_vec().as_slice()).unwrap();
    assert_eq!(decoded, activation);

    let result = kernel::RunActivationResult {
        activation_id: b"act-1".to_vec(),
        outcome: Some(kernel::run_activation_result::Outcome::Commands(
            kernel::Commands {
                commands: vec![
                    kernel::Command {
                        seq: 0,
                        satisfied: true,
                        what: Some(kernel::command::What::RunTask(kernel::RunTask {
                            task_type: "charge".into(),
                            ..Default::default()
                        })),
                    },
                    kernel::Command {
                        seq: 1,
                        satisfied: false,
                        what: Some(kernel::command::What::StartTimer(kernel::StartTimer {
                            duration: Some(orcher_proto::prost_types::Duration {
                                seconds: 30,
                                nanos: 0,
                            }),
                        })),
                    },
                ],
            },
        )),
        queries: vec![],
        updates: vec![],
    };
    let decoded = kernel::RunActivationResult::decode(result.encode_to_vec().as_slice()).unwrap();
    assert_eq!(decoded, result);
}

/// The kernel protocol is its own package: the engine API's types stay at the
/// crate root, unshadowed.
#[test]
fn the_engine_api_is_unchanged_at_the_root() {
    let _engine_payload = orcher_proto::Payload::default();
    let _engine_retry = orcher_proto::RetryPolicy::default();
    let _kernel_retry = kernel::RetryPolicy::default();
}
