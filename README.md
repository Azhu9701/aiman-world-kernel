# AIMAN World Kernel

**One Reality, Many Projections.** AIMAN's open-source multi-World runtime kernel in Rust, with PostgreSQL as the canonical-store direction.

## Why Kernel

Robotics, Labor, Manufacturing, Social, Ecology and History describe overlapping reality. Separate domain truth databases create identity, evidence and temporal drift. A kernel resolves shared canonical references under explicit semantic demand and policy, then hydrates multiple World views. Reality has no World owner.

## Spec vs Runtime and repository relationships

| Repository | Responsibility |
| --- | --- |
| [industry-world-model](https://github.com/Azhu9701/industry-world-model) | Portable protocol, contribution/review semantics and domain vocabulary; also contains an existing reference API/runtime. The protocol authority stays there. |
| [aiman-world-kernel](https://github.com/Azhu9701/aiman-world-kernel) | Independent executable multi-World runtime contracts, resolution and projections. Does not copy or replace protocol schemas. |
| constitutional-ai-os | Separate governance layer, if adopted by a host. No implementation or dependency is imported; Kernel offers policy hooks only. |
| [AIMAN.World / emibot](https://github.com/Azhu9701/emibot) | Product and current production implementation; future host/adapter integration requires separate review. |

This is **v0.1 runnable skeleton**, not a production-ready database service. Production-grade reliability is the direction; no production adapter, deployment, database migration or release is included.

## Run

Install stable Rust, then:

```sh
cargo run -p kernel-api --example multi_world
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

The synthetic example resolves one robot deployment into Robotics, Labor and Social views. Each known field preserves `assertion_ref`, `evidence_refs` and `status`. Robotics is complete for its required location field; Labor and Social remain incomplete because outcomes/voice gaps are unknown. Optional payload remains unknown too. No model or external service is called.

## Current capabilities

- Eight small workspace crates with no HTTP framework or database driver dependency.
- Validated World IDs, vocabulary, semantic keys, projection contracts and acyclic dependency edges; JSON pack loading is atomic.
- SemanticRegistry rejects a second key for the same registered concept; hosts must curate concept identifiers and units (it cannot automatically detect linguistic synonyms).
- RealityResolver trait and immutable in-memory reference fixtures; explicit as-of time, half-open validity, evidence visibility and admission gates.
- PostgreSQL snapshot-reader adapter boundary; no built-in SQL connection or schema installation.
- Required/optional demand, affected-projection closure and deterministic hydration/recompute. Required unknowns mark incomplete; they never receive guessed values.
- Missing, unsupported, invisible, pending, conflicting, superseded and out-of-time fields fail closed. Default policy denies all reads.
- Read query and candidate-submission contracts; no canonical public writer.

## Workspace

`kernel-core` owns portable types; `world-registry` pack validation; `policy` deployment-supplied read/admission hooks; `reality-resolver` reference resolution; `projection-engine` demand and invalidation; `store-postgres` canonical snapshot boundary; `query` read abstraction; `kernel-api` library facade and runnable demo.

Worlds live in `worlds/`; drafts in `specs/`; the executable example lives in `crates/kernel-api/examples/` (see `examples/README.md`).

## Explicit non-goals

No second truth store, public canonical writes, production schema copy, live PostgreSQL integration, Neo4j, governance engine, political judgment, LLM inference, AgentDock/ACP coupling, durable projection cache, background scheduler, identity merger, migration, release or deployment. Projection dependencies are invalidation edges, not a source of new facts. Events/relations/states use shared assertion envelopes; specialized reducers are deferred.

Social extensions are illustrative attributed structures, not an assessment of political positions or social outcomes. Runtime fixtures are synthetic.

## Design and integration

Read [architecture](docs/architecture.md), [protocol boundary](docs/spec-runtime-boundary.md), [EmiBot integration](docs/migration-from-emibot.md) and [spec drafts](specs/README.md). Licensed under Apache-2.0.
