use axum::extract::Query;
use sea_orm::EntityTrait;
use sea_query::Condition;
use uuid::Uuid;

use crate::{init::db, result::{AppResult, AppError}};

pub mod article;
pub mod category;
pub mod comment;
pub mod menu;
pub mod resource;
pub mod role;
pub mod tag;
pub mod user;
pub mod util;

// pub async fn data_scope(user_id: Uuid) -> AppResult<Condition> {
//     let roles = entity::user::Entity::find_by_id(user_id).one(db()).await?.ok_or(AppError::Forbidden)?;
//         .find_with_related(entity::role::Entity)
//         .all(db())
//         .await?;
//     // let filter=Condition::all().add(

//     // )
//     todo!()
// }
