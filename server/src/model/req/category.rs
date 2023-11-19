use garde::Validate;
use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};
use uuid::Uuid;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateCategory {
    #[garde(range(min=1))]
    pub forum_id:i64,
    #[garde(length(min = 1))]
    pub name: String,
}

pub struct CreateCategoryQuery{
    pub auth_user:i64,
    pub forum_id:i64,
    pub name: String,
}
#[derive(Debug, Validate, Deserialize, IntoParams)]
#[into_params(parameter_in= Query)]
pub struct GetCategoryList{
    #[garde(range(min = 1))]
    pub page_size: u64,
    #[garde(range(min = 1))]
    pub page: u64,
    #[garde(custom(is_good_orderby))]
    #[serde(default="default_orderby")]
    pub orderby:String,
    #[garde(skip)]
    #[serde(default)]
    pub order: bool,
    #[garde(skip)]
    #[serde(default)]
    pub name: String,
    #[garde(skip)]
    #[serde(default)]
    pub desc: String,
}
fn is_good_orderby(value: &str,_context:&()) -> garde::Result {
    match value {
        "name"|"updated_at"|"created_at"|"article_count" => {
            return Ok(());
        }
        _ => {
            return Err(garde::Error::new(format!("{value} is not allowed as orderby")));
        }
    }
}
fn default_orderby()->String{
    "created_at".to_string()
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UpdateCategory{
    #[garde(length(min=1))]
    pub name:String
}

pub struct UpdateCategoryQuery{
    pub id:Uuid,
    pub name:String,
}