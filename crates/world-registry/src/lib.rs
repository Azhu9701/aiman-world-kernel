use kernel_core::*;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Default)]
pub struct WorldRegistry {
    worlds: BTreeMap<WorldId, WorldPack>,
}
impl WorldRegistry {
    pub fn load_json(&mut self, json: &str, semantics: &SemanticRegistry) -> Result<()> {
        let pack = serde_json::from_str(json).map_err(|e| KernelError::Invalid(e.to_string()))?;
        self.register(pack, semantics)
    }
    pub fn register(&mut self, pack: WorldPack, semantics: &SemanticRegistry) -> Result<()> {
        let existing: BTreeSet<_> = self.contracts().map(|c| c.id.clone()).collect();
        let local: BTreeSet<_> = pack.projections.iter().map(|c| c.id.clone()).collect();
        if self.worlds.contains_key(&pack.id)
            || pack.entity_types.is_empty()
            || pack
                .entity_types
                .iter()
                .any(|t| SemanticKey::new(t).is_err())
            || pack.projections.is_empty()
            || local.len() != pack.projections.len()
            || !existing.is_disjoint(&local)
            || pack.semantic_keys.iter().any(|k| !semantics.contains(k))
        {
            return Err(KernelError::Invalid(
                "invalid or duplicate world vocabulary".into(),
            ));
        }
        for c in &pack.projections {
            if c.world != pack.id
                || c.entity_types.is_empty()
                || !c.entity_types.is_subset(&pack.entity_types)
                || !c.required.is_disjoint(&c.optional)
                || !c.demand().is_subset(&pack.semantic_keys)
                || c.depends_on
                    .iter()
                    .any(|d| !existing.contains(d) && !local.contains(d))
            {
                return Err(KernelError::Invalid("invalid projection contract".into()));
            }
        }
        let all: BTreeMap<_, _> = self
            .contracts()
            .chain(pack.projections.iter())
            .map(|c| (c.id.clone(), c))
            .collect();
        fn visit(
            id: &ProjectionId,
            all: &BTreeMap<ProjectionId, &ProjectionContract>,
            path: &mut BTreeSet<ProjectionId>,
            done: &mut BTreeSet<ProjectionId>,
        ) -> Result<()> {
            if done.contains(id) {
                return Ok(());
            }
            if !path.insert(id.clone()) {
                return Err(KernelError::Invalid("projection cycle".into()));
            }
            for dep in &all[id].depends_on {
                visit(dep, all, path, done)?;
            }
            path.remove(id);
            done.insert(id.clone());
            Ok(())
        }
        let mut done = BTreeSet::new();
        for id in all.keys() {
            visit(id, &all, &mut BTreeSet::new(), &mut done)?;
        }
        self.worlds.insert(pack.id.clone(), pack);
        Ok(())
    }
    pub fn contracts(&self) -> impl Iterator<Item = &ProjectionContract> {
        self.worlds.values().flat_map(|w| &w.projections)
    }
}
