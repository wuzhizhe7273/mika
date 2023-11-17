use chrono::{DateTime, FixedOffset};
use sea_orm::FromQueryResult;
use serde::Serialize;
use uuid::Uuid;


#[derive(FromQueryResult,Serialize)]
pub struct CategoryVO{
    pub id:Uuid,
    pub name:String,
    pub created_at:DateTime<FixedOffset>,
    pub updated_at:DateTime<FixedOffset>,
    pub article_count:i64
}