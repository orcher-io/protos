//! The host service is generated as a server and a client that talk to each
//! other over a real socket: a host started with a configuration hands out
//! work, refuses a second start and a stale conditional write, and comes back
//! empty once it is shut down.

use std::collections::HashMap;
use std::sync::Mutex;

use orcher_proto::host::v1 as host;
use orcher_proto::host::v1::host_service_client::HostServiceClient;
use orcher_proto::host::v1::host_service_server::{HostService, HostServiceServer};
use orcher_proto::kernel::v1 as kernel;
use tonic::{Code, Request, Response, Status};

/// A host with no engine behind it: one activation to hand out, actor state in
/// memory, and the version of each key counted up on every write.
#[derive(Default)]
struct FakeHost {
    identity: Mutex<Option<String>>,
    activation: Mutex<Option<kernel::RunActivation>>,
    answers: Mutex<Vec<kernel::RunActivationResult>>,
    state: Mutex<HashMap<String, host::StateValue>>,
    stopped: Mutex<bool>,
}

fn scope_key(scope: Option<&host::StateScope>, state_key: &str) -> String {
    let scope = scope.cloned().unwrap_or_default();
    format!("{}/{}/{state_key}", scope.actor_name, scope.key)
}

#[tonic::async_trait]
impl HostService for FakeHost {
    async fn start(
        &self,
        request: Request<host::StartRequest>,
    ) -> Result<Response<host::StartResponse>, Status> {
        let config = request.into_inner().config.unwrap_or_default();
        let mut identity = self.identity.lock().unwrap();
        if identity.is_some() {
            return Err(Status::failed_precondition("the host is already started"));
        }
        let given = if config.identity.is_empty() {
            format!("generated@{}", config.task_queue)
        } else {
            config.identity
        };
        *identity = Some(given.clone());
        Ok(Response::new(host::StartResponse { identity: given }))
    }

    async fn next_activation(
        &self,
        _: Request<host::NextActivationRequest>,
    ) -> Result<Response<host::NextActivationResponse>, Status> {
        let activation = if *self.stopped.lock().unwrap() {
            None
        } else {
            self.activation.lock().unwrap().take()
        };
        Ok(Response::new(host::NextActivationResponse { activation }))
    }

    async fn complete_activation(
        &self,
        request: Request<host::CompleteActivationRequest>,
    ) -> Result<Response<host::CompleteActivationResponse>, Status> {
        let result = request
            .into_inner()
            .result
            .ok_or_else(|| Status::invalid_argument("no result"))?;
        self.answers.lock().unwrap().push(result);
        Ok(Response::new(host::CompleteActivationResponse {}))
    }

    async fn next_task(
        &self,
        _: Request<host::NextTaskRequest>,
    ) -> Result<Response<host::NextTaskResponse>, Status> {
        Ok(Response::new(host::NextTaskResponse { task: None }))
    }

    async fn heartbeat_task(
        &self,
        _: Request<host::HeartbeatTaskRequest>,
    ) -> Result<Response<host::HeartbeatTaskResponse>, Status> {
        Ok(Response::new(host::HeartbeatTaskResponse {
            answer: Some(host::HeartbeatAnswer {
                cancel_requested: false,
            }),
        }))
    }

    async fn wait_task_cancelled(
        &self,
        _: Request<host::WaitTaskCancelledRequest>,
    ) -> Result<Response<host::WaitTaskCancelledResponse>, Status> {
        Ok(Response::new(host::WaitTaskCancelledResponse {
            cancelled: false,
        }))
    }

    async fn complete_task(
        &self,
        _: Request<host::CompleteTaskRequest>,
    ) -> Result<Response<host::CompleteTaskResponse>, Status> {
        Ok(Response::new(host::CompleteTaskResponse {}))
    }

    async fn next_actor_operation(
        &self,
        _: Request<host::NextActorOperationRequest>,
    ) -> Result<Response<host::NextActorOperationResponse>, Status> {
        Ok(Response::new(host::NextActorOperationResponse {
            operation: None,
        }))
    }

    async fn complete_actor_operation(
        &self,
        _: Request<host::CompleteActorOperationRequest>,
    ) -> Result<Response<host::CompleteActorOperationResponse>, Status> {
        Ok(Response::new(host::CompleteActorOperationResponse {}))
    }

    async fn get_actor_state(
        &self,
        request: Request<host::GetActorStateRequest>,
    ) -> Result<Response<host::GetActorStateResponse>, Status> {
        let request = request.into_inner();
        let key = scope_key(request.scope.as_ref(), &request.state_key);
        let value = self.state.lock().unwrap().get(&key).cloned();
        Ok(Response::new(host::GetActorStateResponse { value }))
    }

    async fn set_actor_state(
        &self,
        request: Request<host::SetActorStateRequest>,
    ) -> Result<Response<host::SetActorStateResponse>, Status> {
        let request = request.into_inner();
        let key = scope_key(request.scope.as_ref(), &request.state_key);
        let mut state = self.state.lock().unwrap();
        let stored = state.get(&key).map(|value| value.version.clone());
        if let Some(expected) = &request.expected_version {
            if stored.as_ref() != Some(expected) {
                return Err(Status::failed_precondition(
                    "the stored version has moved on",
                ));
            }
        }
        let next = stored.map_or(1, |version| version.parse::<u64>().unwrap() + 1);
        let version = next.to_string();
        state.insert(
            key,
            host::StateValue {
                value: request.value,
                version: version.clone(),
            },
        );
        Ok(Response::new(host::SetActorStateResponse { version }))
    }

    async fn delete_actor_state(
        &self,
        request: Request<host::DeleteActorStateRequest>,
    ) -> Result<Response<host::DeleteActorStateResponse>, Status> {
        let request = request.into_inner();
        let key = scope_key(request.scope.as_ref(), &request.state_key);
        let deleted = self.state.lock().unwrap().remove(&key).is_some();
        Ok(Response::new(host::DeleteActorStateResponse { deleted }))
    }

    async fn list_actor_state_keys(
        &self,
        request: Request<host::ListActorStateKeysRequest>,
    ) -> Result<Response<host::ListActorStateKeysResponse>, Status> {
        let request = request.into_inner();
        let prefix = scope_key(request.scope.as_ref(), &request.prefix);
        let base = scope_key(request.scope.as_ref(), "");
        let mut keys: Vec<String> = self
            .state
            .lock()
            .unwrap()
            .keys()
            .filter(|key| key.starts_with(&prefix))
            .map(|key| key[base.len()..].to_string())
            .collect();
        keys.sort();
        Ok(Response::new(host::ListActorStateKeysResponse { keys }))
    }

    async fn shutdown(
        &self,
        _: Request<host::ShutdownRequest>,
    ) -> Result<Response<host::ShutdownResponse>, Status> {
        *self.stopped.lock().unwrap() = true;
        Ok(Response::new(host::ShutdownResponse {}))
    }
}

/// Serves `host` on a loopback port and returns a client connected to it.
async fn serve(host: FakeHost) -> HostServiceClient<tonic::transport::Channel> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(HostServiceServer::new(host))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    HostServiceClient::connect(format!("http://{address}"))
        .await
        .unwrap()
}

fn scope() -> host::StateScope {
    host::StateScope {
        actor_name: "Cart".into(),
        key: "cart-7".into(),
        execution_id: "op-1".into(),
    }
}

#[tokio::test]
async fn a_language_runs_a_worker_through_the_host_service() {
    let activation = kernel::RunActivation {
        activation_id: b"act-1".to_vec(),
        run: Some(kernel::RunRef {
            workflow_id: "order-7".into(),
            run_id: "run-1".into(),
            workflow_type: "checkout".into(),
            task_queue: "orders".into(),
            namespace: "default".into(),
        }),
        ..Default::default()
    };
    let mut client = serve(FakeHost {
        activation: Mutex::new(Some(activation.clone())),
        ..Default::default()
    })
    .await;

    // Started once, with the configuration and registrations a language
    // sends; the identity it asked for is the one it gets.
    let start = host::StartRequest {
        config: Some(host::HostConfig {
            connection: Some(host::Connection {
                url: "https://engine.example:443".into(),
                api_key: "key".into(),
                tls: Some(host::Tls {
                    domain_name: "engine.example".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            task_queue: "orders".into(),
            identity: "worker-1".into(),
            supported_step_ids: vec![kernel::StepIdScheme::Kernel as i32],
            shutdown_grace: Some(prost_types::Duration {
                seconds: 30,
                nanos: 0,
            }),
            ..Default::default()
        }),
        registrations: Some(host::Registrations {
            workflow_types: vec!["checkout".into()],
            actor_handlers: vec![host::ActorHandler {
                actor_name: "Cart".into(),
                operation: "add_item".into(),
                mode: host::ActorMode::Exclusive as i32,
            }],
            ..Default::default()
        }),
    };
    let started = client.start(start.clone()).await.unwrap().into_inner();
    assert_eq!(started.identity, "worker-1");
    let again = client.start(start).await.unwrap_err();
    assert_eq!(again.code(), Code::FailedPrecondition);

    // The activation arrives whole and is answered.
    let pulled = client
        .next_activation(host::NextActivationRequest {})
        .await
        .unwrap()
        .into_inner()
        .activation;
    assert_eq!(pulled, Some(activation));
    client
        .complete_activation(host::CompleteActivationRequest {
            result: Some(kernel::RunActivationResult {
                activation_id: b"act-1".to_vec(),
                ..Default::default()
            }),
        })
        .await
        .unwrap();

    // Actor state: a write, a conditional write against the right version, a
    // refused one against a stale version, and the keys under a prefix.
    let written = client
        .set_actor_state(host::SetActorStateRequest {
            scope: Some(scope()),
            state_key: "items".into(),
            value: b"[1]".to_vec(),
            expected_version: None,
        })
        .await
        .unwrap()
        .into_inner()
        .version;
    let rewritten = client
        .set_actor_state(host::SetActorStateRequest {
            scope: Some(scope()),
            state_key: "items".into(),
            value: b"[1,2]".to_vec(),
            expected_version: Some(written.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .version;
    assert_ne!(rewritten, written);
    let stale = client
        .set_actor_state(host::SetActorStateRequest {
            scope: Some(scope()),
            state_key: "items".into(),
            value: b"[]".to_vec(),
            expected_version: Some(written),
        })
        .await
        .unwrap_err();
    assert_eq!(stale.code(), Code::FailedPrecondition);
    let stored = client
        .get_actor_state(host::GetActorStateRequest {
            scope: Some(scope()),
            state_key: "items".into(),
        })
        .await
        .unwrap()
        .into_inner()
        .value
        .unwrap();
    assert_eq!(stored.value, b"[1,2]");
    let keys = client
        .list_actor_state_keys(host::ListActorStateKeysRequest {
            scope: Some(scope()),
            prefix: "it".into(),
        })
        .await
        .unwrap()
        .into_inner()
        .keys;
    assert_eq!(keys, ["items"]);

    // Shut down, every pull comes back empty.
    client.shutdown(host::ShutdownRequest {}).await.unwrap();
    let after = client
        .next_activation(host::NextActivationRequest {})
        .await
        .unwrap()
        .into_inner();
    assert_eq!(after.activation, None);
}
