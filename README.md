# orcher-proto

Protocol Buffer definitions for the [ORCHER](https://github.com/orcher-io)
workflow orchestration platform, and the Rust crate generated from them.

These `.proto` files are the source of truth for the ORCHER gRPC API. The
ORCHER engine and every SDK are built from them.

## Status

Pre-1.0. The API is still evolving and may change in incompatible ways
between minor versions. Field numbers of released messages are not reused.

## Proto files

All files are in the `orcher.v1` package, under `proto/`.

| File | Contents |
|------|----------|
| `types.proto` | Shared types: payloads, retry policy, failures, statuses, journal entries and commands |
| `workflow_service.proto` | `WorkflowService`: start, query, update, cancel and reset workflows, and send them events |
| `execution_service.proto` | `ExecutionService`: poll for and report workflow and task work |
| `query_service.proto` | `QueryService`: list, search and inspect workflow executions |
| `actor_service.proto` | `ActorService`: stateful actors addressed by type and key |
| `worker_service.proto` | `WorkerService`: worker registration and heartbeats |
| `namespace_service.proto` | `NamespaceService`: namespace management |

## Rust

```toml
[dependencies]
orcher-proto = "0.1"
tokio = { version = "1", features = ["full"] }
```

The code is generated at build time by a protobuf compiler written in Rust,
so no `protoc` installation is needed.

Everything in `orcher.v1` is re-exported at the crate root:

```rust
use orcher_proto::workflow_service_client::WorkflowServiceClient;
use orcher_proto::StartWorkflowRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WorkflowServiceClient::connect("http://localhost:50051").await?;

    let response = client
        .start_workflow(StartWorkflowRequest {
            workflow_id: "order-1001".into(),
            workflow_type: "OrderProcessing".into(),
            task_queue: "orders".into(),
            namespace: "default".into(),
            input: br#"{"order_id":"1001"}"#.to_vec(),
            ..Default::default()
        })
        .await?;

    println!("started execution {}", response.into_inner().execution_id);
    Ok(())
}
```

Most applications should use an ORCHER SDK rather than these raw bindings.

## Other languages

Generate code from the files in `proto/` with your language's protobuf
toolchain, for example:

```bash
# Go
protoc -Iproto --go_out=. --go-grpc_out=. proto/*.proto

# TypeScript (protobuf-ts)
npx protoc -Iproto --ts_out=src/generated proto/*.proto

# Python (grpcio-tools)
python -m grpc_tools.protoc -Iproto --python_out=. --grpc_python_out=. proto/*.proto
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
