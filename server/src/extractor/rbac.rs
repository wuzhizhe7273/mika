use async_trait::async_trait;
use axum::{extract::FromRequestParts, RequestPartsExt};
use entity::{r_role_rersource, r_user_role_forum, resource, role};
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
        let Claims { id, .. } = parts
            .extract::<Claims>()
            .await
            .map_err(|e| AppError::Unauthorized)?;
        let user_id = entity::user::Entity::find_by_id(id)
            .one(db())
            .await?
            .ok_or_else(|| AppError::Unauthorized)?
            .id;
        let forum_role: Vec<(i64, i16)> = entity::r_user_role_forum::Entity::find()
            .filter(r_user_role_forum::Column::UserId.eq(user_id))
            .select_only()
            .column(role::Column::Id)
            .column(role::Column::Rtype)
            .join(
                JoinType::LeftJoin,
                role::Relation::RUserRoleForum.def().rev(),
            )
            .distinct()
            .into_tuple()
            .all(db())
            .await
            .map_err(|_| AppError::Forbidden)?;
        // 如果是超级用户,则拥有所有权限
        if forum_role.clone().into_iter().any(|(_, rtype)| rtype == 0) {
            return Ok(Rbac(user_id));
        }
        let roles_id = forum_role
            .into_iter()
            .map(|(id, _)| id)
            .collect::<Vec<i64>>();
        let resources = resource::Entity::find()
            .filter(r_role_rersource::Column::RoleId.is_in(roles_id))
            .join(JoinType::LeftJoin, resource::Relation::RRoleRersource.def())
            .all(db())
            .await
            .map_err(|_| AppError::Forbidden)?;
        // 检查资源中是否包含指定权限
        match resources.into_iter().any(|r| r.href == path) {
            true => return Ok(Rbac(user_id)),
            false => return Err(AppError::Forbidden),
        }
    }
}
