use kernel_core::*;
use policy::{ReadContext, ReadPolicy};
use reality_resolver::{RealityResolver, ResolveRequest};
use std::collections::BTreeSet;
use world_registry::WorldRegistry;
#[derive(Debug, serde::Serialize)]
pub struct Projection {
    pub id: ProjectionId,
    pub world: WorldId,
    pub complete: bool,
    pub packet: RealityPacket,
}
pub struct ProjectionEngine<'a> {
    pub registry: &'a WorldRegistry,
}
impl ProjectionEngine<'_> {
    /// Dependencies are invalidation edges; no projection becomes a fact source.
    pub fn affected(&self, changed: &BTreeSet<SemanticKey>) -> BTreeSet<ProjectionId> {
        let mut affected: BTreeSet<_> = self
            .registry
            .contracts()
            .filter(|c| !c.demand().is_disjoint(changed))
            .map(|c| c.id.clone())
            .collect();
        loop {
            let before = affected.len();
            for c in self.registry.contracts() {
                if !c.depends_on.is_disjoint(&affected) {
                    affected.insert(c.id.clone());
                }
            }
            if before == affected.len() {
                return affected;
            }
        }
    }
    pub fn hydrate(
        &self,
        id: &ProjectionId,
        reality: &RealityRef,
        as_of: Timestamp,
        resolver: &dyn RealityResolver,
        context: &ReadContext,
        policy: &dyn ReadPolicy,
    ) -> Result<Projection> {
        let c = self
            .registry
            .contracts()
            .find(|c| &c.id == id)
            .ok_or_else(|| KernelError::Invalid("unknown projection".into()))?;
        let packet = resolver.resolve(
            ResolveRequest {
                reality,
                keys: &c.demand(),
                as_of,
                context,
            },
            policy,
        )?;
        // Treat third-party resolvers as an untrusted boundary: exact demand and identity.
        if &packet.reality != reality
            || packet.as_of != as_of
            || packet.fields.keys().cloned().collect::<BTreeSet<_>>() != c.demand()
        {
            return Err(KernelError::Invalid(
                "resolver violated packet contract".into(),
            ));
        }
        let complete = c
            .required
            .iter()
            .all(|k| matches!(packet.fields.get(k), Some(ResolvedField::Known { .. })));
        Ok(Projection {
            id: c.id.clone(),
            world: c.world.clone(),
            complete,
            packet,
        })
    }
    pub fn recompute(
        &self,
        changed: &BTreeSet<SemanticKey>,
        reality: &RealityRef,
        as_of: Timestamp,
        resolver: &dyn RealityResolver,
        context: &ReadContext,
        policy: &dyn ReadPolicy,
    ) -> Result<Vec<Projection>> {
        self.affected(changed)
            .iter()
            .map(|id| self.hydrate(id, reality, as_of, resolver, context, policy))
            .collect()
    }
}
