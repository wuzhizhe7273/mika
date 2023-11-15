use crate::{
    init::db,
    model::req:: user::RegisterUser,
    result::{AppError, AppResult},
};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
pub struct UserDAO;
impl UserDAO {
    pub async fn create(u: RegisterUser) -> AppResult<Uuid> {
        let user = entity::user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(u.username),
            email: Set(u.email),
            password: Set(u.password),
            ..Default::default()
        };
        let res = entity::user::Entity::insert(user).exec(db()).await?;
        Ok(res.last_insert_id)
    }
    pub async fn delete(ids: Vec<Uuid>) -> AppResult<u64> {
        let res = entity::user::Entity::delete_many()
            .filter(entity::user::Column::Id.is_in(ids))
            .exec(db())
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn get_pwd(email: String) -> AppResult<(Uuid,String)> {
        let user = entity::user::Entity::find()
            .filter(entity::user::Column::Email.eq(email))
            .one(db())
            .await?
            .ok_or_else(|| AppError::NotFound)?;
        Ok((user.id,user.password))
    }
}
