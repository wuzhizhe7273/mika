use garde::Validate;
use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};
use uuid::Uuid;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateArticle{
    #[garde(skip)]
    pub category:Option<i64>,
    #[garde(length(min=1))]
    pub tags:Option<Vec<i64>>,
    #[garde(length(min=1))]
    pub title:String,
    #[garde(skip)]
    #[serde(default)]
    pub desc:String,
    #[garde(length(min=1))]
    pub content:String
}

pub struct CreateArticleQuery{
    pub user_id:i64,
    pub category:Option<i64>,
    pub tags:Option<Vec<i64>>,
    pub title:String,   
    pub desc:String,
    pub content:String,
}

#[derive(Debug, Validate, Deserialize, IntoParams)]
#[into_params(parameter_in= Query)]
pub struct GetArticleList{
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
    pub title: String,
    #[garde(skip)]
    #[serde(default)]
    pub desc: String,
    #[garde(skip)]
    #[serde(default)]
    pub category:Vec<Uuid>,
    #[garde(skip)]
    #[serde(default)]
    pub tag:Vec<Uuid>
}

fn is_good_orderby(value: &str,_context:&()) -> garde::Result {
    match value {
        "name"|"updated_at"|"created_at" => {
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
pub struct UpdateArticle{
    #[garde(length(min=1))]
    pub title:String,
    #[garde(length(min=1))]
    pub desc:String,
    #[garde(length(min=1))]
    pub content:String,
    #[garde(skip)]
    #[serde(default)]
    pub category:Option<i64>,
    #[garde(skip)]
    #[serde(default)]
    pub tags:Vec<i64>
}

pub struct UpdateArticleQuery{
    pub id:i64,
    pub title:String,
    pub desc:String,
    pub content:String,
    pub category:Option<i64>,
    pub tags:Vec<i64>
}