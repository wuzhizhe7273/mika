use axum::{
    extract::{Query, Path},
    response::{IntoResponse, Response},
    Json,
};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::{
    extractor::rbac::Rbac,
    model::req::{
        tag::{CreateTag, GetTagList, UpdateTag, UpdateTagQuery},
        universal::IDs,
    },
    result::AppResult,
    service::tag::TagService,
};

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
    let id = TagService::create(req).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
         "id":id
        })),
    )
        .into_response())
}
#[utoipa::path(get, path = "/api/v1/tag", params(GetTagList))]
pub async fn list(Garde(Query(req)): Garde<Query<GetTagList>>) -> AppResult<Response> {
    let page = TagService::list(req).await?;
    Ok((StatusCode::OK, Json(page)).into_response())
}

#[utoipa::path(
    delete,
    path="/api/v1/tag",
    request_body=IDs,
)]
pub async fn delete(Garde(Json(IDs { ids })): Garde<Json<IDs>>) -> AppResult<Response> {
    let rows = TagService::delete(ids).await?;
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
    path="/api/v1/tag/{id}",
    request_body=UpdateCategory,
    params(
        ("id"=Uuid,Path,description = "tag id")
    )
)]
pub async fn update(
    Path(id): Path<i64>,
    Garde(Json(req)): Garde<Json<UpdateTag>>,
) -> AppResult<Response> {
    let req=UpdateTagQuery{
        id:id,
        name:req.name
    };
    let id=TagService::update(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}