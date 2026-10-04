//! PostgreSQL adapter contract only: no connection, DDL, migrations or writes.
use kernel_core::*;
use policy::ReadContext;
use std::collections::BTreeSet;
/// Implement against canonical PostgreSQL tables or a reviewed compatibility view.
/// Use parameterized SQL and a consistent snapshot. Authorize before returning IDs.
pub trait CanonicalSnapshotReader {
    fn read_snapshot(
        &self,
        reality: &RealityRef,
        keys: &BTreeSet<SemanticKey>,
        as_of: Timestamp,
        context: &ReadContext,
    ) -> Result<CanonicalSnapshot>;
}
pub struct CanonicalSnapshot {
    pub reality: RealityRef,
    pub evidence_refs: BTreeSet<EvidenceRef>,
    pub assertions: Vec<AssertionEnvelope>,
}
/// Logical schema abstraction, not a production table mapping.
pub struct SchemaContract {
    pub identity_scope: IdentityScope,
    pub timestamps: TimestampStorage,
}
pub enum IdentityScope {
    GlobalCanonical,
}
pub enum TimestampStorage {
    PostgresTimestamptz,
}

/// Transient bridge: the reader supplies canonical rows; resolution never persists them.
pub struct PostgresResolver<R> {
    pub reader: R,
    pub semantics: SemanticRegistry,
}
impl<R: CanonicalSnapshotReader> reality_resolver::RealityResolver for PostgresResolver<R> {
    fn resolve(
        &self,
        request: reality_resolver::ResolveRequest<'_>,
        policy: &dyn policy::ReadPolicy,
    ) -> Result<RealityPacket> {
        if request.reality.0.is_nil() || !policy.can_read_reality(request.context, request.reality)
        {
            return Err(KernelError::UnknownIdentity);
        }
        let snapshot = self.reader.read_snapshot(
            request.reality,
            request.keys,
            request.as_of,
            request.context,
        )?;
        if &snapshot.reality != request.reality
            || snapshot
                .assertions
                .iter()
                .any(|a| &a.reality != request.reality)
        {
            return Err(KernelError::Store("snapshot identity mismatch".into()));
        }
        let resolver = reality_resolver::InMemoryResolver::new(
            BTreeSet::from([snapshot.reality]),
            snapshot.evidence_refs,
            snapshot.assertions,
            self.semantics.clone(),
        );
        resolver.resolve(request, policy)
    }
}
