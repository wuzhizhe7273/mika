use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Query;
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::{
    extractor::rbac::Rbac,
    model::req::{
        article::{
            CreateArticle, CreateArticleQuery, GetArticleList, UpdateArticle, UpdateArticleQuery,
        },
        universal::IDs,
    },
    result::AppResult,
    service::article::ArticleService,
};

#[utoipa::path(
    post,
    path="/api/v1/article",
    request_body=CreateArticle,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(
    Rbac(user_id): Rbac,
    Garde(Json(req)): Garde<Json<CreateArticle>>,
) -> AppResult<Response> {
    let req = CreateArticleQuery {
        user_id: user_id,
        category: req.category,
        title: req.title,
        content: req.content,
        tags: req.tags,
        desc: req.desc,
    };
    let id = ArticleService::create(req).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "id":id
        })),
    )
        .into_response())
}

#[utoipa::path(get, path = "/api/v1/article", params(GetArticleList))]
pub async fn list(Garde(Query(req)): Garde<Query<GetArticleList>>) -> AppResult<Response> {
    let page = ArticleService::list(req).await?;
    Ok((StatusCode::OK, Json(page)).into_response())
}

#[utoipa::path(
    delete,
    path="/api/v1/article",
    request_body=IDs
)]
pub async fn delete(Garde(Json(IDs { ids })): Garde<Json<IDs>>) -> AppResult<Response> {
    let rows = ArticleService::delete(ids).await?;
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
    path="/api/v1/article/{id}",
    request_body=UpdateArticle,
    params(
        ("id"=Uuid,Path,description = "category id")
    )
)]
pub async fn update(
    Path(id): Path<i64>,
    Garde(Json(req)): Garde<Json<UpdateArticle>>,
) -> AppResult<Response> {
    let req = UpdateArticleQuery {
        id,
        title: req.title,
        desc: req.desc,
        content: req.content,
        category: req.category,
        tags: req.tags,
    };
    let id = ArticleService::update(req).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "id":id
        })),
    )
        .into_response())
}
