# Architecture

Canonical PostgreSQL snapshot → policy-gated RealityResolver → RealityPacket → ProjectionEngine → World read models.

RealityRef is global and carries no WorldId. WorldPack declares vocabulary and contracts, not facts. EntityRef and EvidenceRef are canonical references supplied by an adapter. Display names never resolve identity. RealityPacket is transient read output at an explicit instant, not a writable database or canonical authority.

## Boundaries

Core is independent of stores and products. Registry depends only on core. Policy is an injected interface with deny-all default. Resolver evaluates semantic demand. Projection engine depends on validated registry and resolver. Store-postgres defines a consistent-snapshot reader and a transient adapter. Kernel-api exposes a library facade; no unauthenticated HTTP server is supplied.

All evidence referenced by a known field must exist and be visible. Admission is separate from epistemic status. Attributed claims/opinions remain attributed if host policy admits them; admission never converts them to accepted facts. Multiple active assertions remain unknown/conflict, even if values agree; arbitration requires a separately reviewed rule. Invisible/conflicting assertions can suppress a value; this conservative under-disclosure is intentional.

Validity uses [from, to). A missing end is explicitly open-ended; missing start is not accepted. Reversed/empty intervals yield unknown. Adapters must distinguish valid time from recorded time and reject unknown timezone or interval semantics. Historical transaction-time reconstruction is not implemented.

Projection dependencies form an acyclic graph and propagate invalidation transitively. Hydration always resolves canonical demand, never reads another projection as truth. Required unknowns mark incomplete, optional unknowns remain explicit. The reference engine recomputes transiently with no persistent cache; a future cache must key identity, as-of, policy scope and canonical revision and invalidate on identity/evidence/policy/time changes too.

## Trust and limitations

A registered concept identifier prevents duplicate field names for that concept; registry validation is structural, not semantic synonym detection. An adapter must attest canonical entity membership, assertion provenance and snapshot consistency. Third-party RealityResolver implementations are trusted security components responsible for policy and provenance; engine checks packet identity/time/key shape but cannot recover concealed evidence metadata. Do not expose an arbitrary resolver to untrusted callers.

World entity types validate contract vocabulary, not inferred canonical classification. Reference fixtures explicitly attest entity references. Production adapters must enforce identity membership/type mapping; v0.1 has no canonical entity classification service. ReadContext principal is supplied by a trusted authenticated host, not caller-asserted identity.
