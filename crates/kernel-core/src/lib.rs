//! Portable runtime contracts, independent of any production schema.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("invalid contract: {0}")]
    Invalid(String),
    #[error("unknown canonical identity")]
    UnknownIdentity,
    #[error("canonical adapter failure: {0}")]
    Store(String),
}
pub type Result<T> = std::result::Result<T, KernelError>;
macro_rules! identifier {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self> {
                let value = value.into();
                if value.is_empty()
                    || value.len() > 128
                    || !value.bytes().all(|b| {
                        b.is_ascii_lowercase()
                            || b.is_ascii_digit()
                            || b == b'.'
                            || b == b'-'
                            || b == b'_'
                    })
                    || !value.as_bytes()[0].is_ascii_lowercase()
                    || value.ends_with('.')
                    || value.contains("..")
                {
                    return Err(KernelError::Invalid(value));
                }
                Ok(Self(value))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl TryFrom<String> for $name {
            type Error = KernelError;
            fn try_from(value: String) -> Result<Self> {
                Self::new(value)
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}
identifier!(WorldId);
identifier!(SemanticKey);
identifier!(ProjectionId);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RealityRef(pub Uuid);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EntityRef(pub Uuid);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EvidenceRef(pub Uuid);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRange {
    pub from: DateTime<Utc>,
    pub to: Option<DateTime<Utc>>,
}
impl TemporalRange {
    pub fn contains(&self, at: DateTime<Utc>) -> bool {
        self.to.is_none_or(|to| to > self.from && at < to) && at >= self.from
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssertionStatus {
    AcceptedFact,
    AttributedClaim,
    AttributedOpinion,
    Pending,
    Conflicting,
    Superseded,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionEnvelope {
    pub assertion_ref: String,
    pub reality: RealityRef,
    pub entities: BTreeSet<EntityRef>,
    pub semantic_key: SemanticKey,
    pub value: Value,
    pub evidence_refs: Vec<EvidenceRef>,
    pub status: AssertionStatus,
    pub valid_time: TemporalRange,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnknownReason {
    Missing,
    PolicyDenied,
    InvalidProvenance,
    InvalidTime,
    Conflict,
    Unadmitted,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "resolution")]
pub enum ResolvedField {
    Known {
        value: Value,
        assertion_ref: String,
        evidence_refs: Vec<EvidenceRef>,
        status: AssertionStatus,
    },
    Unknown {
        reason: UnknownReason,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityPacket {
    pub reality: RealityRef,
    pub entities: BTreeSet<EntityRef>,
    pub as_of: DateTime<Utc>,
    pub fields: BTreeMap<SemanticKey, ResolvedField>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionContract {
    pub id: ProjectionId,
    pub world: WorldId,
    pub entity_types: BTreeSet<String>,
    pub required: BTreeSet<SemanticKey>,
    pub optional: BTreeSet<SemanticKey>,
    pub depends_on: BTreeSet<ProjectionId>,
}
impl ProjectionContract {
    pub fn demand(&self) -> BTreeSet<SemanticKey> {
        self.required.union(&self.optional).cloned().collect()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPack {
    pub id: WorldId,
    pub entity_types: BTreeSet<String>,
    pub semantic_keys: BTreeSet<SemanticKey>,
    pub projections: Vec<ProjectionContract>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticDefinition {
    pub concept: String,
    pub value_type: String,
    pub unit: Option<String>,
}
#[derive(Debug, Default, Clone)]
pub struct SemanticRegistry {
    definitions: BTreeMap<SemanticKey, SemanticDefinition>,
}
impl SemanticRegistry {
    pub fn register(&mut self, key: SemanticKey, definition: SemanticDefinition) -> Result<()> {
        if SemanticKey::new(&definition.concept).is_err()
            || definition
                .unit
                .as_ref()
                .is_some_and(|u| u.trim().is_empty())
            || !["string", "number", "boolean", "object", "array"]
                .contains(&definition.value_type.as_str())
            || self.definitions.contains_key(&key)
            || self
                .definitions
                .values()
                .any(|d| d.concept == definition.concept)
        {
            return Err(KernelError::Invalid(
                "duplicate concept/key or invalid definition".into(),
            ));
        }
        self.definitions.insert(key, definition);
        Ok(())
    }
    pub fn contains(&self, key: &SemanticKey) -> bool {
        self.definitions.contains_key(key)
    }
    pub fn accepts(&self, key: &SemanticKey, value: &Value) -> bool {
        self.definitions
            .get(key)
            .is_some_and(|d| match d.value_type.as_str() {
                "string" => value.is_string(),
                "number" => value.is_number(),
                "boolean" => value.is_boolean(),
                "object" => value.is_object(),
                "array" => value.is_array(),
                _ => false,
            })
    }
}
/// Write side is a proposal boundary only; no canonical public writer is supplied.
pub trait CandidateSink {
    fn propose(&self, candidate: &AssertionEnvelope) -> Result<String>;
}

pub type Timestamp = DateTime<Utc>;
