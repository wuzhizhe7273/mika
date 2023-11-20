use uuid::Uuid;

use crate::{model::req::comment:: CreateCommentQuery, result::AppResult, dao::comment::CommentDAO};

pub struct CommentService;
impl CommentService {
    pub async fn create(req:CreateCommentQuery)->AppResult<i64>{
        CommentDAO::create(req).await
    }
}