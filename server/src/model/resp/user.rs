use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserOptionVO{
    pub id:i64,
    pub username:String
}