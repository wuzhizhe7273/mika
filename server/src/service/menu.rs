use uuid::Uuid;

use crate::{model::req::menu::CreateMenu, result::AppResult, dao::menu::MenuDAO};

pub struct MenuService;
impl MenuService {
    pub async fn create(req:CreateMenu)->AppResult<i64>{
        MenuDAO::create(req).await
    }
}