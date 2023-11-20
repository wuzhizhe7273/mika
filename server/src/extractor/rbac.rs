use async_trait::async_trait;
use axum::{extract::FromRequestParts, RequestPartsExt};
use entity::{r_role_rersource, resource, role};
use http::request::Parts;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, EntityTrait, LoaderTrait, QueryFilter,
    QuerySelect, RelationTrait, Statement,
};
use sea_query::JoinType;

use crate::{extractor::jwt::Claims, init::db, result::AppError};

pub struct Rbac(pub i64);

#[async_trait]
impl<S> FromRequestParts<S> for Rbac
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let path = parts.uri.path().to_string();
        let Claims {id,.. } = parts
            .extract::<Claims>()
            .await
            .map_err(|e| AppError::Unauthorized)?;
       let user=entity::user::Entity::find_by_id(id).one(db()).await?.ok_or(AppError::Unauthorized)?;
       let roles=entity::role::Entity::find()
        .distinct()
        .filter(entity::user::Column::Id.eq(user.id))
        .left_join(entity::user::Entity)
        .all(db())
        .await?;
        // 如果是超级用户,则拥有所有权限
        if roles.clone().into_iter().any(|role|{role.rtype ==0}){
            return Ok(Rbac(id));
        }
        let roles_id=roles.into_iter().map(|role|{role.id}).collect::<Vec<i64>>();
        let resources=entity::resource::Entity::find()
            .filter(entity::role::Column::Id.is_in(roles_id))
            .left_join(entity::role::Entity)
            .all(db())
            .await?;
        if resources.into_iter().any(|res|{res.href==path}) {
            return Ok(Rbac(id));
        }
        Err(AppError::Forbidden)
}
}
