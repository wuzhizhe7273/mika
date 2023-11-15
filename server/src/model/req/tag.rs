use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateTag{
    #[garde(length(min=1))]
    pub name:String
}