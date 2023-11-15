use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateCategory{
    #[garde(length(min=1))]
    pub name:String
}