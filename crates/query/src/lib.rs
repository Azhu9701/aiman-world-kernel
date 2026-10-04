use kernel_core::*;
use policy::ReadContext;
/// A read model is computed output; it cannot admit assertions or mutate truth.
pub trait ReadModelQuery {
    fn read(
        &self,
        projection: &ProjectionId,
        reality: &RealityRef,
        as_of: Timestamp,
        context: &ReadContext,
    ) -> Result<RealityPacket>;
}
