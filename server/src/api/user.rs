use axum::{response::{IntoResponse, Response}, Json, http::StatusCode};
use axum_valid::Garde;
use serde_json::json;

use crate::{model::req::{user::{RegisterUser, LoginUser}, universal::IDs}, result::AppResult, service::user::UserService, extractor::rbac::Rbac};


pub async fn ping()->String{
    "hello world!".to_string()
}
#[utoipa::path(
    post,
    path="/api/v1/register",
    request_body=RegisterUser,
    responses(
        (status=200,description="用户注册成功"),
        (status=500,description="内部错误")
    )
)]
pub async fn register(Garde(Json(req)):Garde<Json<RegisterUser>>) -> AppResult<Response> {
    let id=UserService::register(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "id":id
    }))).into_response())
}
#[utoipa::path(
    delete,
    path="/api/v1/user",
    request_body=IDs,
    responses(
        (status=200,description="用户删除成功"),
        (status=500,description="内部错误")
    ),
    security(
        ("jwt"=[])
    )
)]
pub async fn delete(Rbac(_user_id):Rbac,Garde(Json(ids)):Garde<Json<IDs>>)->AppResult<Response>{
    let rows=UserService::delete(ids.ids).await?;
    Ok((StatusCode::OK,Json(json!({
        "total":rows
    }))).into_response())
}
#[utoipa::path(
    post,
    path="/api/v1/login",
    request_body=LoginUser,
    responses(
        (status=200,description="用户登录成功"),
        (status=500,description="内部错误")
    )
)]
pub async fn login(Garde(Json(req)):Garde<Json<LoginUser>>)->AppResult<Response>{
    let token=UserService::login(req).await?;
    Ok((StatusCode::OK,Json(json!({
        "token":token
    }))).into_response())
}
