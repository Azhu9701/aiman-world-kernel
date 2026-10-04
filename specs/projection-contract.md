# Projection Contract

A projection declares global ProjectionId, owning WorldId, entity type vocabulary, disjoint required/optional semantic keys and dependency IDs. Demand is the union of keys. Changed keys select intersecting contracts; transitive dependents are also affected. Hydration resolves the canonical demand at an explicit as-of time. Every demanded key appears as known or unknown; completeness means every required key is known, not that a deployment succeeded or a social outcome improved.
