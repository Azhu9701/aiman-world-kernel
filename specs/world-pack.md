# World Pack

WorldId is a lowercase bounded identifier. Packs declare nonempty entity types, registered semantic keys and unique projections. Each projection belongs to the pack, uses declared entity types and keys, and references existing or same-pack dependency IDs. Missing dependencies and cycles reject the whole registration; load does not partially mutate registry. Cross-world dependencies require the referenced pack to be registered first. Packs never own Reality or canonical evidence.
