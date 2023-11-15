use axum::{response::{Response, IntoResponse}, Json};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

use crate::{result::AppResult, model::req::resource::CreateResource, service::resource::ResourceService, extractor::rbac::Rbac};

#[utoipa::path(
    post,
    path="/api/v1/resource",
    request_body=CreateResource,
    security(
        ("jwt"=[])
    )
)]
pub async fn create(Rbac(_user_id):Rbac,Garde(Json(req)):Garde<Json<CreateResource>>)->AppResult<Response>{
    let id=ResourceService::create(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}