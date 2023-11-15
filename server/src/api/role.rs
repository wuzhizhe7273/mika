use axum::{response::{Response, IntoResponse}, Json};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

use crate::{result::AppResult, service::role::RoleService, model::req::role::CreateRole, extractor::rbac::Rbac};

#[utoipa::path(
    post,
    path="/api/v1/role",
    request_body=CreateRole,
    security(
        ("jwt"=[])
    )
)]
pub async fn create(Rbac(_user_id):Rbac,Garde(Json(req)):Garde<Json<CreateRole>>)->AppResult<Response>{
    let id=RoleService::create(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}