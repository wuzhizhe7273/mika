use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateMenu{
    #[garde(length(min=1))]
    pub name:String,
    #[garde(length(min=1))]
    pub menu_type:String,
    #[garde(skip)]
    #[serde(default)]
    pub desc:String,
    #[garde(skip)]
    pub parent_id:Option<i64>,
    #[garde(skip)]
    #[serde(default)]
    pub order:i32
}