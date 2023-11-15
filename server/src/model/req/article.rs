use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateArticle{
    #[garde(skip)]
    pub category:Option<Uuid>,
    #[garde(length(min=1))]
    pub tags:Option<Vec<Uuid>>,
    #[garde(length(min=1))]
    pub title:String,
    #[garde(skip)]
    #[serde(default)]
    pub desc:String,
    #[garde(length(min=1))]
    pub content:String
}

pub struct CreateArticleQuery{
    pub user_id:Uuid,
    pub category:Option<Uuid>,
    pub tags:Option<Vec<Uuid>>,
    pub title:String,   
    pub desc:String,
    pub content:String,
}