use crate::{extractor::rbac::Rbac, model::req:: category::CreateCategory, result::AppResult, service::category::CategoryService};
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

#[utoipa::path(
    post,
    path="/api/v1/category",
    request_body=CreateCategory,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(
    Rbac(_user_id): Rbac,
    Garde(Json(req)): Garde<Json<CreateCategory>>,
) -> AppResult<Response> {
    let id=CategoryService::create(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}
