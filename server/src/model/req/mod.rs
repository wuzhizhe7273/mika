pub mod user;
pub mod universal;
pub mod category;
pub mod role;
pub mod resource;
pub mod tag;
pub mod comment;
pub mod article;
pub mod menu;

pub struct PermQuery<T>{
    pub auth:i64,
    pub req:T
}