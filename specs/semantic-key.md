# Semantic Key

Use shared concept keys such as physical.payload, not robot.payload/selection.payload/labor.payload. SemanticRegistry owns a curated concept identifier, value type and optional unit. Duplicate key or concept IDs are rejected. Value type is checked during resolution. A numeric value uses the registered unit; adapters must normalize units explicitly and reject uncertain conversion. No string similarity or model inference identifies equivalent concepts. Registration changes require review and future version negotiation.
