use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateResource{
    #[garde(length(min=1))]
    pub name:String,
    #[garde(length(min=1))]
    pub href:String,
    #[garde(skip)]
    #[serde(default)]
    pub desc:String,
    #[garde(skip)]
    pub method:String
}