use sea_orm::{Set, EntityTrait};
use uuid::Uuid;

use crate::{model::req::comment::CreateCommentQuery, result::AppResult, init::db};

pub struct  CommentDAO;
impl CommentDAO {
    pub async fn create(req:CreateCommentQuery)->AppResult<Uuid>{
        let c=entity::comment::ActiveModel{
            id:Set(Uuid::new_v4()),
            parent_id:Set(req.parent_id),
            user_id:Set(req.user_id),
            content:Set(req.content),
            ..Default::default()
        };
        let id=entity::comment::Entity::insert(c).exec(db()).await?.last_insert_id;
        Ok(id)
    }
}