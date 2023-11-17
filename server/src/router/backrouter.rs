use axum::{
    routing::{delete, get, post,put},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api, util::apidoc::ApiDoc};

pub async fn create() -> Router {
    Router::new()
        .route("/ping", get(api::user::ping))
        .route("/api/v1/register", post(api::user::register))
        .route("/api/v1/login", post(api::user::login))
        .route("/api/v1/user", delete(api::user::delete))
        .route("/api/v1/resource", post(api::resource::create))
        .route("/api/v1/role", post(api::role::create))
        .route(
            "/api/v1/category",
            post(api::category::create)
                .get(api::category::list)
                .delete(api::category::delete)
        )
        .route("/api/v1/category/:id", put(api::category::update))
        .route("/api/v1/tag", post(api::tag::create).get(api::tag::list))
        .route("/api/v1/comment", post(api::comment::create))
        .route("/api/v1/article", post(api::article::create).get(api::article::list).delete(api::article::delete))
        .route("/api/v1/menu", post(api::menu::create))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
