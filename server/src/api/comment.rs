use axum::{response::{Response, IntoResponse}, Json};
use axum_valid::Garde;
use http::StatusCode;
use serde_json::json;

use crate::{result::AppResult, extractor::rbac::Rbac, model::req::comment::{CreateComment, CreateCommentQuery}, service::comment::CommentService};


#[utoipa::path(
    post,
    path="/api/v1/comment",
    request_body=CreateComment,
    security(
        ("jwt"= [] )
    )
)]
pub async fn create(Rbac(user_id):Rbac,Garde(Json(req)):Garde<Json<CreateComment>>)->AppResult<Response>{
    let c=CreateCommentQuery{
        user_id:user_id,
        parent_id:req.parent_id,
        reply_id:req.reply_id,
        content:req.content
    };
    let id=CommentService::create(c).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}