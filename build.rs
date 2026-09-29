use std::path::PathBuf;

/// The API's proto files, compiled in this order.
const PROTOS: &[&str] = &[
    "proto/types.proto",
    "proto/workflow_service.proto",
    "proto/execution_service.proto",
    "proto/query_service.proto",
    "proto/actor_service.proto",
    "proto/worker_service.proto",
    "proto/namespace_service.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    // protox is a protobuf compiler written in Rust that bundles the Google
    // well-known types, so building this crate (and everything that depends on
    // it) needs no `protoc` and no system include paths.
    let descriptors = protox::compile(PROTOS, ["proto"])?;

    // Generate both server traits (for the server) and client stubs (for the
    // SDKs). Well-known types map to prost-types rather than being generated
    // again, so they interoperate with other prost-based crates.
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::prost_types")
        .out_dir(&out_dir)
        .compile_fds(descriptors)?;

    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
