use uuid::Uuid;

use crate::{init::db, model::req::category::CreateCategory, result::AppResult};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

pub struct CategoryDAO;
impl CategoryDAO {
    pub async fn create(c: CreateCategory) -> AppResult<Uuid> {
        let category = entity::category::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(c.name),
            ..Default::default()
        };
        let res = entity::category::Entity::insert(category)
            .exec(db())
            .await?;
        Ok(res.last_insert_id)
    }

    pub async fn detete(ids: Vec<Uuid>) -> AppResult<u64> {
        let res = entity::category::Entity::delete_many()
            .filter(entity::category::Column::Id.is_in(ids))
            .exec(db())
            .await?;
        Ok(res.rows_affected)
    }
}
