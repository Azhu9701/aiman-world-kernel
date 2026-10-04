use kernel_core::*;
use policy::{ReadContext, ReadPolicy};
use std::collections::{BTreeMap, BTreeSet};
pub struct ResolveRequest<'a> {
    pub reality: &'a RealityRef,
    pub keys: &'a BTreeSet<SemanticKey>,
    pub as_of: Timestamp,
    pub context: &'a ReadContext,
}
pub trait RealityResolver {
    fn resolve(
        &self,
        request: ResolveRequest<'_>,
        policy: &dyn ReadPolicy,
    ) -> Result<RealityPacket>;
}
/// Immutable reference fixture, never a persistent or production truth store.
pub struct InMemoryResolver {
    identities: BTreeSet<RealityRef>,
    evidence: BTreeSet<EvidenceRef>,
    assertions: Vec<AssertionEnvelope>,
    semantics: SemanticRegistry,
}
impl InMemoryResolver {
    pub fn new(
        identities: BTreeSet<RealityRef>,
        evidence: BTreeSet<EvidenceRef>,
        assertions: Vec<AssertionEnvelope>,
        semantics: SemanticRegistry,
    ) -> Self {
        Self {
            identities,
            evidence,
            assertions,
            semantics,
        }
    }
}
impl RealityResolver for InMemoryResolver {
    fn resolve(&self, r: ResolveRequest<'_>, policy: &dyn ReadPolicy) -> Result<RealityPacket> {
        if r.reality.0.is_nil()
            || !self.identities.contains(r.reality)
            || !policy.can_read_reality(r.context, r.reality)
        {
            return Err(KernelError::UnknownIdentity);
        }
        let mut fields = BTreeMap::new();
        let mut entities = BTreeSet::new();
        for key in r.keys {
            let candidates: Vec<_> = self
                .assertions
                .iter()
                .filter(|a| &a.reality == r.reality && &a.semantic_key == key)
                .collect();
            let active: Vec<_> = candidates
                .iter()
                .filter(|a| a.valid_time.contains(r.as_of))
                .copied()
                .collect();
            let field = match active.as_slice() {
                [] => ResolvedField::Unknown {
                    reason: if candidates.is_empty() {
                        UnknownReason::Missing
                    } else {
                        UnknownReason::InvalidTime
                    },
                },
                [a] => {
                    let reason = if a.assertion_ref.trim().is_empty()
                        || a.entities.is_empty()
                        || a.entities.iter().any(|e| e.0.is_nil())
                        || a.evidence_refs.is_empty()
                        || a.evidence_refs
                            .iter()
                            .any(|e| e.0.is_nil() || !self.evidence.contains(e))
                        || !self.semantics.accepts(key, &a.value)
                    {
                        Some(UnknownReason::InvalidProvenance)
                    } else if a
                        .evidence_refs
                        .iter()
                        .any(|e| !policy.can_read_evidence(r.context, e))
                    {
                        Some(UnknownReason::PolicyDenied)
                    } else if matches!(
                        a.status,
                        AssertionStatus::Pending | AssertionStatus::Superseded
                    ) || !policy.admits(r.context, a)
                    {
                        Some(UnknownReason::Unadmitted)
                    } else if a.status == AssertionStatus::Conflicting {
                        Some(UnknownReason::Conflict)
                    } else {
                        None
                    };
                    match reason {
                        Some(reason) => ResolvedField::Unknown { reason },
                        None => {
                            entities.extend(a.entities.iter().cloned());
                            ResolvedField::Known {
                                value: a.value.clone(),
                                assertion_ref: a.assertion_ref.clone(),
                                evidence_refs: a.evidence_refs.clone(),
                                status: a.status.clone(),
                            }
                        }
                    }
                }
                _ => ResolvedField::Unknown {
                    reason: UnknownReason::Conflict,
                },
            };
            fields.insert(key.clone(), field);
        }
        Ok(RealityPacket {
            reality: r.reality.clone(),
            entities,
            as_of: r.as_of,
            fields,
        })
    }
}
