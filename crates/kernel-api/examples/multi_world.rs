use kernel_api::*;
use std::collections::BTreeSet;
use uuid::Uuid;

/// Synthetic demo only. Real deployments must provide authorization policy.
struct FixturePolicy;
impl ReadPolicy for FixturePolicy {
    fn can_read_reality(&self, _: &ReadContext, _: &RealityRef) -> bool {
        true
    }
    fn can_read_evidence(&self, _: &ReadContext, _: &EvidenceRef) -> bool {
        true
    }
    fn admits(&self, _: &ReadContext, a: &AssertionEnvelope) -> bool {
        a.status == AssertionStatus::AcceptedFact
    }
}
fn main() -> Result<()> {
    let mut semantics = SemanticRegistry::default();
    for (key, concept, kind, unit) in [
        ("deployment.location", "deployment-location", "string", None),
        ("physical.payload", "rated-payload", "number", Some("kg")),
        ("labor.outcome", "observed-labor-outcome", "object", None),
        ("social.voice_gap", "documented-voice-gap", "object", None),
    ] {
        semantics.register(
            SemanticKey::new(key)?,
            SemanticDefinition {
                concept: concept.into(),
                value_type: kind.into(),
                unit: unit.map(String::from),
            },
        )?;
    }
    let mut registry = WorldRegistry::default();
    for json in [
        include_str!("../../../worlds/robotics/pack.json"),
        include_str!("../../../worlds/labor/pack.json"),
        include_str!("../../../worlds/social/pack.json"),
    ] {
        registry.load_json(json, &semantics)?;
    }
    let reality = RealityRef(Uuid::from_u128(1));
    let evidence = EvidenceRef(Uuid::from_u128(2));
    let at = "2026-10-01T00:00:00Z"
        .parse::<Timestamp>()
        .map_err(|e| KernelError::Invalid(e.to_string()))?;
    let key = SemanticKey::new("deployment.location")?;
    let assertion = AssertionEnvelope {
        assertion_ref: "event:fixture-deployment".into(),
        reality: reality.clone(),
        entities: BTreeSet::from([EntityRef(Uuid::from_u128(3)), EntityRef(Uuid::from_u128(4))]),
        semantic_key: key.clone(),
        value: serde_json::json!("Synthetic Factory A"),
        evidence_refs: vec![evidence.clone()],
        status: AssertionStatus::AcceptedFact,
        valid_time: TemporalRange { from: at, to: None },
    };
    let resolver = InMemoryResolver::new(
        BTreeSet::from([reality.clone()]),
        BTreeSet::from([evidence]),
        vec![assertion],
        semantics,
    );
    let projections = ProjectionEngine {
        registry: &registry,
    }
    .recompute(
        &BTreeSet::from([key]),
        &reality,
        at,
        &resolver,
        &ReadContext { principal: None },
        &FixturePolicy,
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&projections)
            .map_err(|e| KernelError::Invalid(e.to_string()))?
    );
    Ok(())
}
