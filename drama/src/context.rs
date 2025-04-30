use crate::{Classification, TenantId};

#[derive(Clone, Debug)]
pub(crate) struct Context {
    tenant_id: TenantId,
    classification: Classification,
}
