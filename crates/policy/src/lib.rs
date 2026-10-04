//! Deployments provide authorization; default policy denies everything.
use kernel_core::*;
#[derive(Debug, Clone)]
pub struct ReadContext {
    pub principal: Option<String>,
}
pub trait ReadPolicy {
    fn can_read_reality(&self, context: &ReadContext, reality: &RealityRef) -> bool;
    fn can_read_evidence(&self, context: &ReadContext, evidence: &EvidenceRef) -> bool;
    fn admits(&self, context: &ReadContext, assertion: &AssertionEnvelope) -> bool;
}
pub struct DenyAll;
impl ReadPolicy for DenyAll {
    fn can_read_reality(&self, _: &ReadContext, _: &RealityRef) -> bool {
        false
    }
    fn can_read_evidence(&self, _: &ReadContext, _: &EvidenceRef) -> bool {
        false
    }
    fn admits(&self, _: &ReadContext, _: &AssertionEnvelope) -> bool {
        false
    }
}
