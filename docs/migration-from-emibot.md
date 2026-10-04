# Future EmiBot adapter (no migration performed)

Inspected remote main: `eab6a8e62c0d26f48cbd121270f82cf090b278dc` on 2026-10-04. Relevant owning code: `emibot-rs/src/services/world_core.rs`, `src/routes/world_read_model.rs`, and their canonical assertion/evidence read paths.

Current core has shared AssertionEnvelope, knowledge-status mapping, entity type/canonical-name references and approved evidence-backed relation guards. Read routes resolve approved aliases, reject ambiguity and enforce source visibility. Timeline, relation and state SQL remain product/schema-specific. This snapshot does not prove production deployment state.

## Integration seam

Implement CanonicalSnapshotReader in the host, behind existing authorization and AppError/SQLx boundaries. Keep parameterized SQL, a consistent PostgreSQL snapshot and existing visibility rules. Supply a global RealityRef and canonical EntityRef mapping; do not derive identity from display names. Map assertion kinds/statuses explicitly and preserve canonical assertion references and evidence IDs.

Map product columns to curated SemanticRegistry concepts with explicit units. Deployment location and labor outcome are separate concepts. Denormalized timeline/graph views do not become new canonical assertions. Admission does not flatten subject claims/opinions into facts.

Normalize tstzrange/timestamp/timestamptz only after examining actual schema semantics; naive timestamps require a documented timezone and cannot be assumed UTC. Missing validity must remain unknown. CanonicalSnapshotReader must authorize identity and evidence existence before disclosure, and supply complete active candidate sets, including unresolved conflicts.

First integration should be read-only shadow comparison: evaluate the same authorized identities/time windows, compare provenance/status/unknowns with existing API, and add compatibility tests. Only then separately approve production connection and deployment. Write-side review stays in EmiBot; CandidateSink is a future proposal interface, not a writer.

No existing repository files, production tables, schemas, credentials or deployment configuration were changed for this skeleton.
