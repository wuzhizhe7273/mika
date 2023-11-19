use sea_orm::DatabaseConnection;

use crate::result::AppResult;

pub async fn valid(db:DatabaseConnection,user_id:i64,forum_ids:Vec<i64>)->AppResult<()>{

    todo!()
}