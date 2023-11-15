use uuid::Uuid;

use crate::{model::req::menu::CreateMenu, result::AppResult, dao::menu::MenuDAO};

pub struct MenuService;
impl MenuService {
    pub async fn create(req:CreateMenu)->AppResult<Uuid>{
        MenuDAO::create(req).await
    }
}