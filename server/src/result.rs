use axum::{
    http::{header::WWW_AUTHENTICATE, StatusCode},
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type AppResult<T>=Result<T,AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("authentication required")]
    Unauthorized,
    #[error("user may not perform that action")]
    Forbidden,
    #[error("request path not found")]
    NotFound,
    #[error("an error occurred with the database")]
    Database(#[from] sea_orm::DbErr),
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Database(_) |Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    [(WWW_AUTHENTICATE, "Token")],
                    self.to_string(),
                )
                    .into_response();
            }
            Self::Forbidden => {
                return (self.status_code(), self.to_string()).into_response();
            }
            Self::NotFound => {
                return (self.status_code(),self.to_string()).into_response();
            },
            Self::Database(ref e)=>{
                tracing::error!("Database error:{:?}",e);
                return  (self.status_code(),self.to_string()).into_response();
            },
            Self::Anyhow(ref e) =>{
                tracing::error!("Generic error: {:?}", e);
                return  (self.status_code(),self.to_string()).into_response();
            },
        }
    }
}
