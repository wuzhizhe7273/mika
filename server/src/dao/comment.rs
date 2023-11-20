use sea_orm::{Set, EntityTrait};
use uuid::Uuid;

use crate::{model::req::comment::CreateCommentQuery, result::AppResult, init::db};

pub struct  CommentDAO;
impl CommentDAO {
    pub async fn create(req:CreateCommentQuery)->AppResult<i64>{
        let c=entity::comment::ActiveModel{
            parent_id:Set(req.parent_id),
            user_id:Set(req.user_id),
            article_id:Set(req.article_id),
            content:Set(req.content),
            ..Default::default()
        };
        let id=entity::comment::Entity::insert(c).exec(db()).await?.last_insert_id;
        Ok(id)
    }
}