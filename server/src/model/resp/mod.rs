pub mod category;
pub mod tag;
pub mod article;
pub mod user;
use serde::Serialize;

#[derive(Serialize)]
pub struct Pagination<T>{
    pub total:u64,
    pub page:u64,
    pub data:Vec<T>
}