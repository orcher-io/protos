//! # ORCHER Protocol Definitions
//!
//! Rust bindings for the ORCHER gRPC API, generated at build time from the
//! `.proto` files in `proto/` with tonic-build. Every message, enum, client
//! and server of the `orcher.v1` package is re-exported at the crate root.
//!
//! ## Services
//!
//! - **WorkflowService**: client API for starting and managing workflows
//! - **ExecutionService**: API services use to poll for and run workflow and task work
//! - **QueryService**: API for listing, searching and inspecting workflow executions
//! - **ActorService**: actors, stateful objects addressed by type and key
//! - **WorkerService**: worker registration, heartbeats and deregistration
//! - **NamespaceService**: namespace management
//!
//! The kernel protocol, the exchange between the kernel (sdk-core) and a
//! language SDK, is in [`kernel::v1`].
//!
//! ## Terminology
//!
//! - **Service**: an execution runtime that polls for and runs work
//! - **Task**: a unit of work run outside the workflow, where side effects belong
//! - **Event**: an external message sent to a running workflow
//! - **Execution Journal**: the event-sourced history of a workflow execution

// Re-exported so users build against the same versions the bindings use.
pub use prost;
pub use prost_types;
pub use tonic;

/// ORCHER protocol definitions, generated from the `.proto` files.
pub mod orcher {
    // Generated code is not ours to restyle, and new clippy releases add lints
    // it trips (for example `result_large_err` on every gRPC method).
    #[allow(clippy::all)]
    pub mod v1 {
        tonic::include_proto!("orcher.v1");
    }

    /// The kernel protocol (`orcher.kernel.v1`): the exchange between the
    /// kernel (sdk-core) and a language SDK, separate from the engine's API
    /// above and sharing no types with it. Not re-exported at the crate root,
    /// so its names never shadow the engine API's.
    pub mod kernel {
        #[allow(clippy::all)]
        pub mod v1 {
            tonic::include_proto!("orcher.kernel.v1");
        }
    }
}

pub use orcher::v1::*;

/// The kernel protocol, as `orcher_proto::kernel::v1`.
pub use orcher::kernel;

/// Version of the protocol definitions in this crate.
pub const PROTOCOL_VERSION: &str = env!("CARGO_PKG_VERSION");
