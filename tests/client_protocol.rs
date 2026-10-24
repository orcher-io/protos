//! The client protocol's messages are generated and round-trip as protobuf.

use orcher_proto::client::v1 as client;
use orcher_proto::prost::Message;

/// A start, with its options, and its answer survive encoding and decoding
/// unchanged; so does a failed call's answer.
#[test]
fn a_call_and_its_answers_round_trip() {
    let call = client::ClientCall {
        call: Some(client::client_call::Call::StartWorkflow(
            client::StartWorkflow {
                workflow_id: "order-7".into(),
                workflow_type: "checkout".into(),
                task_queue: "orders".into(),
                input: br#"{"n":1}"#.to_vec(),
                retry_policy: Some(client::RetryPolicy {
                    max_attempts: 3,
                    ..Default::default()
                }),
                id_reuse_policy: client::IdReusePolicy::RejectDuplicate as i32,
                ..Default::default()
            },
        )),
        namespace: "payments".into(),
        timeout: Some(orcher_proto::prost_types::Duration {
            seconds: 5,
            nanos: 0,
        }),
    };
    assert_eq!(
        client::ClientCall::decode(call.encode_to_vec().as_slice()).unwrap(),
        call
    );

    for answer in [
        client::ClientAnswer {
            answer: Some(client::client_answer::Answer::Started(client::Started {
                workflow: Some(client::WorkflowRef {
                    workflow_id: "order-7".into(),
                    run_id: "run-1".into(),
                }),
            })),
        },
        client::ClientAnswer {
            answer: Some(client::client_answer::Answer::Error(client::ClientError {
                code: client::ErrorCode::WorkflowAlreadyExists as i32,
                message: "in use".into(),
                retryable: false,
                run_id: "run-1".into(),
                status: client::WorkflowStatus::Unspecified as i32,
            })),
        },
    ] {
        assert_eq!(
            client::ClientAnswer::decode(answer.encode_to_vec().as_slice()).unwrap(),
            answer
        );
    }
}
