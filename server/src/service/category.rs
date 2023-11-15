use uuid::Uuid;

use crate::{result::AppResult, model::req::category::CreateCategory, dao::category::CategoryDAO};

pub struct CategoryService;
impl CategoryService {
    pub async fn create(req:CreateCategory)->AppResult<Uuid>{
        CategoryDAO::create(req).await
    }
}