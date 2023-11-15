use axum::{response::{Response, IntoResponse}, Json};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

use crate::{result::AppResult, extractor::rbac::Rbac, model::req::article::{CreateArticle, CreateArticleQuery}, service::article::ArticleService};

#[utoipa::path(
    post,
    path="/api/v1/article",
    request_body=CreateArticle,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(Rbac(user_id):Rbac,Garde(Json(req)):Garde<Json<CreateArticle>>)->AppResult<Response>{
    let req=CreateArticleQuery{
        user_id:user_id,
        category:req.category,
        title:req.title,
        content:req.content,
        tags:req.tags,
        desc:req.desc
    };
    let id=ArticleService::create(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}