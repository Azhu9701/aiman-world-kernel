# Reality Packet

Transient read output contains global RealityRef, disclosed canonical EntityRefs, explicit as-of timestamp and demanded fields. Known fields preserve value, assertion_ref, evidence_refs and assertion status. Unknown fields contain a reason and disclose no assertion/evidence identifiers or values. Packet is not an ingest envelope and must never be written back as canonical truth.

Only canonical identity, valid temporal range, existing visible evidence and admitted status permit known output. Missing fields stay unknown. Pending/superseded candidates are not admitted, conflicts are not resolved by recency, and time uses [from,to). The adapter must return complete canonical candidates; the resolver cannot detect candidates omitted by its store.
