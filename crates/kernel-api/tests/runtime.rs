use kernel_api::*;
use policy::DenyAll;
use std::collections::BTreeSet;
use uuid::Uuid;
fn key(s: &str) -> SemanticKey {
    SemanticKey::new(s).unwrap()
}
fn at() -> Timestamp {
    "2026-10-01T00:00:00Z".parse().unwrap()
}
fn reality() -> RealityRef {
    RealityRef(Uuid::from_u128(1))
}
fn evidence() -> EvidenceRef {
    EvidenceRef(Uuid::from_u128(2))
}
fn semantics() -> SemanticRegistry {
    let mut s = SemanticRegistry::default();
    for (k, t) in [
        ("deployment.location", "string"),
        ("physical.payload", "number"),
        ("labor.outcome", "object"),
        ("social.voice_gap", "object"),
    ] {
        s.register(
            key(k),
            SemanticDefinition {
                concept: k.into(),
                value_type: t.into(),
                unit: None,
            },
        )
        .unwrap();
    }
    s
}
fn registry() -> WorldRegistry {
    let mut r = WorldRegistry::default();
    for json in [
        include_str!("../../../worlds/robotics/pack.json"),
        include_str!("../../../worlds/labor/pack.json"),
        include_str!("../../../worlds/social/pack.json"),
    ] {
        r.load_json(json, &semantics()).unwrap();
    }
    r
}
fn assertion() -> AssertionEnvelope {
    AssertionEnvelope {
        assertion_ref: "event:deployment-1".into(),
        reality: reality(),
        entities: BTreeSet::from([EntityRef(Uuid::from_u128(3))]),
        semantic_key: key("deployment.location"),
        value: serde_json::json!("Factory A"),
        evidence_refs: vec![evidence()],
        status: AssertionStatus::AcceptedFact,
        valid_time: TemporalRange {
            from: at(),
            to: None,
        },
    }
}
struct Allow {
    evidence: bool,
}
impl ReadPolicy for Allow {
    fn can_read_reality(&self, _: &ReadContext, _: &RealityRef) -> bool {
        true
    }
    fn can_read_evidence(&self, _: &ReadContext, _: &EvidenceRef) -> bool {
        self.evidence
    }
    fn admits(&self, _: &ReadContext, a: &AssertionEnvelope) -> bool {
        a.status == AssertionStatus::AcceptedFact
    }
}
fn resolve(assertions: Vec<AssertionEnvelope>, allow: bool) -> ResolvedField {
    let resolver = InMemoryResolver::new(
        BTreeSet::from([reality()]),
        BTreeSet::from([evidence()]),
        assertions,
        semantics(),
    );
    resolver
        .resolve(
            ResolveRequest {
                reality: &reality(),
                keys: &BTreeSet::from([key("deployment.location")]),
                as_of: at(),
                context: &ReadContext { principal: None },
            },
            &Allow { evidence: allow },
        )
        .unwrap()
        .fields
        .remove(&key("deployment.location"))
        .unwrap()
}
#[test]
fn one_reality_three_projections_preserve_provenance_and_unknown() {
    let registry = registry();
    let resolver = InMemoryResolver::new(
        BTreeSet::from([reality()]),
        BTreeSet::from([evidence()]),
        vec![assertion()],
        semantics(),
    );
    let engine = ProjectionEngine {
        registry: &registry,
    };
    let projections = engine
        .recompute(
            &BTreeSet::from([key("deployment.location")]),
            &reality(),
            at(),
            &resolver,
            &ReadContext { principal: None },
            &Allow { evidence: true },
        )
        .unwrap();
    assert_eq!(projections.len(), 3);
    for p in &projections {
        assert_eq!(p.packet.reality, reality());
        match &p.packet.fields[&key("deployment.location")] {
            ResolvedField::Known {
                assertion_ref,
                evidence_refs,
                status,
                ..
            } => {
                assert_eq!(assertion_ref, "event:deployment-1");
                assert_eq!(evidence_refs, &vec![evidence()]);
                assert_eq!(status, &AssertionStatus::AcceptedFact);
            }
            _ => panic!("expected canonical reference"),
        }
        if p.world.as_str() != "robotics" {
            assert!(!p.complete);
        }
        for (k, v) in &p.packet.fields {
            if k != &key("deployment.location") {
                assert!(matches!(
                    v,
                    ResolvedField::Unknown {
                        reason: UnknownReason::Missing
                    }
                ));
            }
        }
    }
    assert!(engine
        .affected(&BTreeSet::from([key("unrelated.key")]))
        .is_empty());
}
#[test]
fn fail_closed_visibility_time_provenance_conflicts_and_status() {
    assert!(matches!(
        resolve(vec![assertion()], false),
        ResolvedField::Unknown {
            reason: UnknownReason::PolicyDenied
        }
    ));
    assert!(matches!(
        resolve(vec![assertion(), assertion()], true),
        ResolvedField::Unknown {
            reason: UnknownReason::Conflict
        }
    ));
    let mut a = assertion();
    a.evidence_refs.clear();
    assert!(matches!(
        resolve(vec![a], true),
        ResolvedField::Unknown {
            reason: UnknownReason::InvalidProvenance
        }
    ));
    let mut a = assertion();
    a.evidence_refs = vec![EvidenceRef(Uuid::from_u128(99))];
    assert!(matches!(
        resolve(vec![a], true),
        ResolvedField::Unknown {
            reason: UnknownReason::InvalidProvenance
        }
    ));
    let mut a = assertion();
    a.value = serde_json::json!(55);
    assert!(matches!(
        resolve(vec![a], true),
        ResolvedField::Unknown {
            reason: UnknownReason::InvalidProvenance
        }
    ));
    let mut a = assertion();
    a.valid_time.to = Some(at());
    assert!(matches!(
        resolve(vec![a], true),
        ResolvedField::Unknown {
            reason: UnknownReason::InvalidTime
        }
    ));
    let mut a = assertion();
    a.status = AssertionStatus::Pending;
    assert!(matches!(
        resolve(vec![a], true),
        ResolvedField::Unknown {
            reason: UnknownReason::Unadmitted
        }
    ));
}
#[test]
fn unknown_identity_and_default_policy_do_not_leak() {
    let resolver = InMemoryResolver::new(
        BTreeSet::from([reality()]),
        BTreeSet::from([evidence()]),
        vec![assertion()],
        semantics(),
    );
    for id in [reality(), RealityRef(Uuid::from_u128(999))] {
        assert!(matches!(
            resolver.resolve(
                ResolveRequest {
                    reality: &id,
                    keys: &BTreeSet::from([key("deployment.location")]),
                    as_of: at(),
                    context: &ReadContext { principal: None }
                },
                &DenyAll
            ),
            Err(KernelError::UnknownIdentity)
        ));
    }
}
#[test]
fn semantic_concepts_cannot_be_forked_and_bad_pack_load_is_atomic() {
    let mut s = semantics();
    assert!(s
        .register(
            key("selection.payload"),
            SemanticDefinition {
                concept: "physical.payload".into(),
                value_type: "number".into(),
                unit: None
            }
        )
        .is_err());
    assert!(SemanticKey::new("Bad Key").is_err());
    assert!(serde_json::from_str::<WorldId>("\"../invalid\"").is_err());
    let mut r = registry();
    let before = r.contracts().count();
    assert!(r
        .load_json(include_str!("../../../worlds/robotics/pack.json"), &s)
        .is_err());
    assert_eq!(r.contracts().count(), before);
}
#[test]
fn dependency_invalidation_is_transitive_and_cycles_rejected() {
    let mut r = registry();
    let pack = |cycle: bool| WorldPack {
        id: WorldId::new("history").unwrap(),
        entity_types: BTreeSet::from(["robot".into()]),
        semantic_keys: BTreeSet::from([key("labor.outcome")]),
        projections: vec![ProjectionContract {
            id: ProjectionId::new("history.summary").unwrap(),
            world: WorldId::new("history").unwrap(),
            entity_types: BTreeSet::from(["robot".into()]),
            required: BTreeSet::from([key("labor.outcome")]),
            optional: BTreeSet::new(),
            depends_on: BTreeSet::from([ProjectionId::new(if cycle {
                "history.summary"
            } else {
                "robotics.deployment"
            })
            .unwrap()]),
        }],
    };
    assert!(r.register(pack(true), &semantics()).is_err());
    r.register(pack(false), &semantics()).unwrap();
    assert!(ProjectionEngine { registry: &r }
        .affected(&BTreeSet::from([key("physical.payload")]))
        .contains(&ProjectionId::new("history.summary").unwrap()));
}

#[test]
fn invalid_world_contracts_are_rejected_without_partial_registration() {
    let source = include_str!("../../../worlds/robotics/pack.json");
    for edit in 0..5 {
        let mut pack: WorldPack = serde_json::from_str(source).unwrap();
        match edit {
            0 => {
                pack.semantic_keys.insert(key("unregistered.key"));
            }
            1 => {
                pack.projections[0].entity_types.insert("undeclared".into());
            }
            2 => {
                pack.projections[0]
                    .optional
                    .insert(key("deployment.location"));
            }
            3 => {
                pack.projections[0]
                    .depends_on
                    .insert(ProjectionId::new("missing.projection").unwrap());
            }
            _ => {
                pack.projections[0].world = WorldId::new("other").unwrap();
            }
        }
        let mut registry = WorldRegistry::default();
        assert!(registry.register(pack, &semantics()).is_err());
        assert_eq!(registry.contracts().count(), 0);
    }
}
