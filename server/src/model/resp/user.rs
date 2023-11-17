use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserOptionVO{
    pub id:Uuid,
    pub username:String
}