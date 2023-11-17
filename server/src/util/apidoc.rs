use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::model::req::{
    universal::IDs,
    user::{LoginUser, RegisterUser},
};
use crate::{
    api::{article, category, comment, menu, resource, role, tag, user},
    model::req::{
        article::CreateArticle,
        category::{CreateCategory, UpdateCategory},
        comment::CreateComment,
        menu::CreateMenu,
        resource::CreateResource,
        role::CreateRole,
        tag::CreateTag,
    },
};
#[derive(OpenApi)]
#[openapi(
    paths(
        user::register,
        user::delete,
        user::login,
        resource::create,
        role::create,
        category::create,
        category::list,
        category::delete,
        category::update,
        tag::create,
        comment::create,
        article::create,
        menu::create

    ),
    components(schemas(
        RegisterUser,
        LoginUser,
        CreateRole,
        CreateResource,
        CreateCategory,
        UpdateCategory,
        CreateTag,
        CreateComment,
        CreateArticle,
        CreateMenu,
        IDs,
    )),
    modifiers(&SecurityAddon),
    tags((name="blog",description="blog api"))
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
