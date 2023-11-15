use uuid::Uuid;

use crate::{model::req::comment:: CreateCommentQuery, result::AppResult, dao::comment::CommentDAO};

pub struct CommentService;
impl CommentService {
    pub async fn create(req:CreateCommentQuery)->AppResult<Uuid>{
        CommentDAO::create(req).await
    }
}