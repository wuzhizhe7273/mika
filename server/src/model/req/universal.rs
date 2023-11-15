use utoipa::ToSchema;
use uuid::Uuid;
use serde::Deserialize;
use garde::Validate;

#[derive(Debug,Deserialize,Validate,ToSchema)]
pub struct IDs{
    #[garde(length(min=1))]
    pub ids:Vec<Uuid>
}