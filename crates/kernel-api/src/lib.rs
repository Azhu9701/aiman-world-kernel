//! Library facade; no public write or HTTP endpoint in v0.1.
pub use kernel_core::*;
pub use policy::{ReadContext, ReadPolicy};
pub use projection_engine::{Projection, ProjectionEngine};
pub use reality_resolver::{InMemoryResolver, RealityResolver, ResolveRequest};
pub use world_registry::WorldRegistry;
