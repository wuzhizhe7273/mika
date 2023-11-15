use async_trait::async_trait;
use axum::{extract::FromRequestParts, RequestPartsExt};
use http::request::Parts;
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, LoaderTrait, Statement};
use uuid::Uuid;

use crate::{extractor::jwt::Claims, init::db, result::AppError};

pub struct Rbac(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for Rbac
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let path = parts.uri.path().to_string();
        tracing::debug!("path:{}",path);
        let Claims { id, .. } = parts
            .extract::<Claims>()
            .await
            .map_err(|e| AppError::Unauthorized)?;
        tracing::debug!("decode jwt success");
        let user = entity::user::Entity::find_by_id(id)
            .one(db())
            .await?
            .ok_or_else(|| AppError::Unauthorized)?
            .id;
        let backend: sea_orm::DatabaseBackend = db().get_database_backend();
        let roles = match backend {
            DatabaseBackend::Postgres => entity::role::Entity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    backend,
                    r#"
                    WITH RECURSIVE roles(id) AS (
                        SELECT role.* FROM role JOIN r_user_role ON role.id = r_user_role.role_id WHERE r_user_role.user_id=$1
                        UNION ALL
                        SELECT role.* FROM role INNER JOIN roles r ON role.parent_id=r.id
                    )
                    SELECT * FROM roles;
            "#,
                    [user.into()],
                ))
                .all(db())
                .await
                .map_err(|e| AppError::Forbidden),
            DatabaseBackend::Sqlite | DatabaseBackend::MySql => entity::role::Entity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    backend,
                    r#"
                    WITH RECURSIVE roles(id) AS (
                        SELECT role.* FROM role JOIN r_user_role ON role.id = r_user_role.role_id WHERE r_user_role.user_id=?
                        UNION ALL
                        SELECT role.* FROM role INNER JOIN roles r ON role.parent_id=r.id
                    )
                    SELECT * FROM roles;
            "#,
                    [user.into()],
                ))
                .all(db())
                .await
                .map_err(|e| AppError::Forbidden),
        }?;
        // 如果是超级用户,则拥有所有权限
        if roles.clone().into_iter().any(|r|{r.name=="SuperUser"}) {
            return Ok(Rbac(user));
        }
        //根据角色加载资源
        let resources = roles
            .load_many_to_many(
                entity::resource::Entity,
                entity::r_role_rersource::Entity, 
                db(),
            )
            .await
            .map_err(|e| AppError::Forbidden)?;
        // 检查资源中是否包含指定权限
        for resource in resources {
            match resource.into_iter().any(|r| r.href == path) {
                true => {
                    return Ok(Rbac(user));
                }
                false => {}
            }
        }
        Err(AppError::Forbidden)
    }
}
