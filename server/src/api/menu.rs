use axum::{response::{Response, IntoResponse}, Json};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

use crate::{result::AppResult, extractor::rbac::Rbac, model::req::menu::CreateMenu, service::menu::MenuService};


#[utoipa::path(
    post,
    path="/api/v1/menu",
    request_body=CreateMenu,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(
    Rbac(_user_id):Rbac,
    Garde(Json(req)):Garde<Json<CreateMenu>>
)->AppResult<Response>{
    let id=MenuService::create(req).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "id":id
        }))
    ).into_response())
}