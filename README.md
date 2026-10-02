<p>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./assets/banner.svg">
    <source media="(prefers-color-scheme: light)" srcset="./assets/banner-light.svg">
    <img alt="ORCHER Protocol" src="./assets/banner.svg" width="100%">
  </picture>
</p>

<p align="center"><sub>The ORCHER API, defined once in Protocol Buffers and shared by the engine and every SDK.</sub></p>

<br />

<div>
  <a href="https://crates.io/crates/orcher-proto"><img src="https://img.shields.io/crates/v/orcher-proto?style=flat-square&labelColor=0a0a0a&color=04B385&logo=rust&logoColor=white" alt="crates.io"></a>
  <a href="https://docs.rs/orcher-proto"><img src="https://img.shields.io/docsrs/orcher-proto?style=flat-square&labelColor=0a0a0a&color=38BDF0&logo=docsdotrs&logoColor=white" alt="docs.rs"></a>
  <a href="https://github.com/orcher-io/protos/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/orcher-io/protos/ci.yml?branch=main&style=flat-square&labelColor=0a0a0a&color=04B385&logo=github&logoColor=white&label=CI" alt="CI"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-Apache_2.0-38BDF0?style=flat-square&labelColor=0a0a0a" alt="Apache 2.0"></a>
</div>

<br />

The `.proto` files are the source of truth for the ORCHER gRPC API, and `orcher-proto` is the Rust crate generated from them.

- <img height="14" src="https://octicons-col.vercel.app/file-code/38BDF0"> **One package**: every service and message lives in `orcher.v1`, under `proto/`
- <img height="14" src="https://octicons-col.vercel.app/package/38BDF0"> **Rust bindings**: tonic clients and servers, re-exported at the crate root
- <img height="14" src="https://octicons-col.vercel.app/tools/38BDF0"> **No `protoc` needed**: the crate compiles the files with a protobuf compiler written in Rust
- <img height="14" src="https://octicons-col.vercel.app/code/38BDF0"> **Any language**: generate clients for Go, TypeScript, Python and more from the same files
- <img height="14" src="https://octicons-col.vercel.app/shield-check/38BDF0"> **Compatible by rule**: CI rejects any wire- or JSON-breaking change, and field numbers are never reused

<br />

### <img height="16" src="https://octicons-col.vercel.app/download/38BDF0"> Install

```toml
[dependencies]
orcher-proto = "0.1"
tokio = { version = "1", features = ["full"] }
```

> [!NOTE]
> These are raw bindings. To write workflows, use an SDK instead: [`orcher-sdk`](https://crates.io/crates/orcher-sdk) for Rust or [`@orcher/sdk`](https://www.npmjs.com/package/@orcher/sdk) for TypeScript. The API is pre-1.0 and may change between minor releases; every change is listed in [CHANGELOG.md](CHANGELOG.md).

<br />

### <img height="16" src="https://octicons-col.vercel.app/play/38BDF0"> Use from Rust

Everything in `orcher.v1` is re-exported at the crate root, along with the `tonic`, `prost` and `prost-types` versions the bindings were built with:

```rust
use orcher_proto::workflow_service_client::WorkflowServiceClient;
use orcher_proto::StartWorkflowRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WorkflowServiceClient::connect("http://localhost:50051").await?;

    let response = client
        .start_workflow(StartWorkflowRequest {
            workflow_id: "order-1001".into(),
            workflow_type: "confirm_order".into(),
            task_queue: "orders".into(),
            namespace: "default".into(),
            input: br#"{"id":"order-1001"}"#.to_vec(),
            ..Default::default()
        })
        .await?;

    println!("started execution {}", response.into_inner().execution_id);
    Ok(())
}
```

<br />

### <img height="16" src="https://octicons-col.vercel.app/terminal/38BDF0"> Other languages

Generate code from the files in `proto/` with your language's protobuf toolchain:

```bash
# Go
protoc -Iproto --go_out=. --go-grpc_out=. proto/*.proto

# TypeScript (protobuf-ts)
npx protoc -Iproto --ts_out=src/generated proto/*.proto

# Python (grpcio-tools)
python -m grpc_tools.protoc -Iproto --python_out=. --grpc_python_out=. proto/*.proto
```

<br />

### <img height="16" src="https://octicons-col.vercel.app/file-directory/38BDF0"> Proto files

| File | Contents |
|------|----------|
| `types.proto` | Shared types: payloads, retry policy, failures, statuses, journal entries and commands |
| `workflow_service.proto` | `WorkflowService`: start, query, update, cancel and reset workflows, and send them events |
| `execution_service.proto` | `ExecutionService`: poll for and report workflow and task work |
| `query_service.proto` | `QueryService`: list, search and inspect workflow executions |
| `actor_service.proto` | `ActorService`: stateful actors addressed by type and key |
| `worker_service.proto` | `WorkerService`: worker registration and heartbeats |
| `namespace_service.proto` | `NamespaceService`: namespace management |

<br />

### <img height="16" src="https://octicons-col.vercel.app/stack/38BDF0"> How it fits

| Layer | Package | Repository |
|-------|---------|------------|
| API definitions | [`orcher-proto`](https://crates.io/crates/orcher-proto) | [orcher-io/protos](https://github.com/orcher-io/protos) |
| SDK core | [`orcher-sdk-core`](https://crates.io/crates/orcher-sdk-core) | [orcher-io/sdk-core](https://github.com/orcher-io/sdk-core) |
| Rust SDK | [`orcher-sdk`](https://crates.io/crates/orcher-sdk) | [orcher-io/sdk-rust](https://github.com/orcher-io/sdk-rust) |
| TypeScript SDK | [`@orcher/sdk`](https://www.npmjs.com/package/@orcher/sdk) | [orcher-io/sdk-ts](https://github.com/orcher-io/sdk-ts) |

The engine serves this API, `orcher-sdk-core` speaks it on behalf of every language SDK, and the SDKs give it an idiomatic face.

<br />

### <img height="16" src="https://octicons-col.vercel.app/heart/38BDF0"> Contributing

Issues and pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for how to build, test and propose a change.

### <img height="16" src="https://octicons-col.vercel.app/law/38BDF0"> License

Licensed under the [Apache License, Version 2.0](LICENSE).
