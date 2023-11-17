use chrono::{DateTime, FixedOffset};
use sea_orm::{FromQueryResult, DerivePartialModel};
use serde::Serialize;
use uuid::Uuid;
use entity::tag::Entity as Tag;


#[derive(FromQueryResult,Serialize)]
pub struct TagVO{
    pub id:Uuid,
    pub name:String,
    pub created_at:DateTime<FixedOffset>,
    pub updated_at:DateTime<FixedOffset>,
    pub article_count:i64
}


#[derive(FromQueryResult,DerivePartialModel,Serialize)]
#[sea_orm(entity="Tag")]
pub struct TagOptionVO{
    pub id:Uuid,
    pub name:String
}