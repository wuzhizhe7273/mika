use axum::{Json, response::{Response, IntoResponse}};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

use crate::{extractor::rbac::Rbac, model::req::tag::CreateTag, result::AppResult, service:: tag::TagService};

#[utoipa::path(
    post,
    path="/api/v1/tag",
    request_body=CreateTag,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(
    Rbac(_user_id): Rbac,
    Garde(Json(req)): Garde<Json<CreateTag>>,
) -> AppResult<Response> {
   let id=TagService::create(req).await?;
   Ok((StatusCode::OK,Json(json!({
    "id":id
   }))).into_response())
}