use kernel_core::*;
use policy::{DenyAll, ReadContext, ReadPolicy};
use reality_resolver::{RealityResolver, ResolveRequest};
use std::{cell::Cell, collections::BTreeSet};
use store_postgres::*;
struct Reader {
    called: Cell<bool>,
    mismatch: bool,
    fail: bool,
}
impl CanonicalSnapshotReader for Reader {
    fn read_snapshot(
        &self,
        reality: &RealityRef,
        _: &BTreeSet<SemanticKey>,
        _: Timestamp,
        _: &ReadContext,
    ) -> Result<CanonicalSnapshot> {
        self.called.set(true);
        if self.fail {
            return Err(KernelError::Store("unavailable".into()));
        }
        Ok(CanonicalSnapshot {
            reality: if self.mismatch {
                RealityRef(Default::default())
            } else {
                reality.clone()
            },
            evidence_refs: BTreeSet::new(),
            assertions: vec![],
        })
    }
}
struct IdentityOnly;
impl ReadPolicy for IdentityOnly {
    fn can_read_reality(&self, _: &ReadContext, _: &RealityRef) -> bool {
        true
    }
    fn can_read_evidence(&self, _: &ReadContext, _: &EvidenceRef) -> bool {
        false
    }
    fn admits(&self, _: &ReadContext, _: &AssertionEnvelope) -> bool {
        false
    }
}
#[test]
fn denied_identity_never_reaches_reader() {
    let resolver = PostgresResolver {
        reader: Reader {
            called: Cell::new(false),
            mismatch: false,
            fail: false,
        },
        semantics: SemanticRegistry::default(),
    };
    let reality = RealityRef("00000000-0000-0000-0000-000000000001".parse().unwrap());
    assert!(matches!(
        resolver.resolve(
            ResolveRequest {
                reality: &reality,
                keys: &BTreeSet::new(),
                as_of: "2026-10-01T00:00:00Z".parse().unwrap(),
                context: &ReadContext { principal: None }
            },
            &DenyAll
        ),
        Err(KernelError::UnknownIdentity)
    ));
    assert!(!resolver.reader.called.get());
}
#[test]
fn mismatch_and_adapter_failure_propagate_without_success_packet() {
    let reality = RealityRef("00000000-0000-0000-0000-000000000001".parse().unwrap());
    for (mismatch, fail) in [(true, false), (false, true)] {
        let resolver = PostgresResolver {
            reader: Reader {
                called: Cell::new(false),
                mismatch,
                fail,
            },
            semantics: SemanticRegistry::default(),
        };
        assert!(matches!(
            resolver.resolve(
                ResolveRequest {
                    reality: &reality,
                    keys: &BTreeSet::new(),
                    as_of: "2026-10-01T00:00:00Z".parse().unwrap(),
                    context: &ReadContext { principal: None }
                },
                &IdentityOnly
            ),
            Err(KernelError::Store(_))
        ));
    }
}
