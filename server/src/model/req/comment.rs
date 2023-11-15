use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct CreateComment{
   #[garde(skip)]
   pub parent_id:Option<Uuid>,
   #[garde(skip)]
   pub reply_id:Option<Uuid>,
   #[garde(length(min=1))]
   pub content:String
}

pub struct CreateCommentQuery{
    pub user_id:Uuid,
    pub parent_id:Option<Uuid>,
    pub reply_id:Option<Uuid>,
    pub content:String
}