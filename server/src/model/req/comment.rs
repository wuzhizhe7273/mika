use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateComment{
   #[garde(skip)]
   pub parent_id:Option<i64>,
   #[garde(skip)]
   pub reply_id:Option<i64>,
   #[garde(length(min=1))]
   pub content:String,
   #[garde(skip)]
   pub article_id:i64
}

pub struct CreateCommentQuery{
    pub user_id:i64,
    pub parent_id:Option<i64>,
    pub reply_id:Option<i64>,
    pub content:String,
    pub article_id:i64
}