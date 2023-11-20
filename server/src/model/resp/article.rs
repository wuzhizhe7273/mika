use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use uuid::Uuid;

use super::{user::UserOptionVO, category::CategoryOptionVO, tag::TagOptionVO};


#[derive(Serialize)]
pub struct ArticleVO{
    pub id:i64,
    pub title:String,
    pub user:Option<UserOptionVO>,
    pub category:Option<CategoryOptionVO>,
    pub tags:Option<Vec<TagOptionVO>>,
    pub desc:String,
    pub content:Option<String>,
    pub updated_at:DateTime<FixedOffset>,
    pub created_at:DateTime<FixedOffset>
}