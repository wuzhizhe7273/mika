use crate::{
    extractor::rbac::Rbac,
    model::req::{
        category::{CreateCategory, GetCategoryList, UpdateCategory, UpdateCategoryQuery, CreateCategoryQuery},
        universal::IDs, PermQuery,
    },
    result::AppResult,
    service::category::CategoryService,
};
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Json,
};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

#[utoipa::path(
    post,
    path="/api/v1/category",
    request_body=CreateCategory,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(
    Rbac(user_id): Rbac,
    Garde(Json(req)): Garde<Json<CreateCategory>>,
) -> AppResult<Response> {
    let query=PermQuery{
        auth:user_id,
        req:req
    };
    let id = CategoryService::create(query).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "id":id
        })),
    )
        .into_response())
}
#[utoipa::path(get, path = "/api/v1/category", params(GetCategoryList))]
pub async fn list(Garde(Query(req)): Garde<Query<GetCategoryList>>) -> AppResult<Response> {
    let page = CategoryService::list(req).await?;
    Ok((StatusCode::OK, Json(page)).into_response())
}
#[utoipa::path(
    delete,
    path="/api/v1/category",
    request_body=IDs
)]
pub async fn delete(Garde(Json(req)): Garde<Json<IDs>>) -> AppResult<Response> {
    let rows = CategoryService::delete(req.ids).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "rows":rows
        })),
    )
        .into_response())
}
#[utoipa::path(
    put,
    path="/api/v1/category/{id}",
    request_body=UpdateCategory,
    params(
        ("id"=Uuid,Path,description = "category id")
    )
)]
pub async fn update(
    Path(id): Path<i64>,
    Garde(Json(req)): Garde<Json<UpdateCategory>>,
) -> AppResult<Response> {
    let req=UpdateCategoryQuery{
        id,
        name:req.name
    };
    let id=CategoryService::update(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}
