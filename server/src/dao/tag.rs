use sea_orm::{Set, EntityTrait};
use uuid::Uuid;

use crate::{model::req::tag::CreateTag, result::AppResult, init::db};

pub struct TagDAO;
impl TagDAO {
    pub async fn create(req: CreateTag) -> AppResult<Uuid> {
        let tag = entity::tag::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
            ..Default::default()
        };
        let id=entity::tag::Entity::insert(tag).exec(db()).await?.last_insert_id;
        Ok(id)
    }
}
