use sea_orm::{EntityTrait, Set};
use uuid::Uuid;

use crate::{
    init::db,
    model::req:: resource::CreateResource,
    result::AppResult,
};

pub struct ResourceDAO;
impl ResourceDAO {
    pub async fn create(req: CreateResource) -> AppResult<i64> {
        let resource = entity::resource::ActiveModel {
            name: Set(req.name),
            method: Set(req.method),
            href: Set(req.href),
            ..Default::default()
        };
        let id = entity::resource::Entity::insert(resource)
            .exec(db())
            .await?
            .last_insert_id;
        Ok(id)
    }
}
