use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateRole{
    #[garde(length(min=1))]
    pub name:String,
    #[garde(skip)]
    #[serde(default)]
    pub desc:String,
    #[garde(skip)]
    pub parent:Option<Uuid>,
    #[garde(length(min=1))]
    pub resource:Option<Vec<i64>>
}