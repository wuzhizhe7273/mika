use uuid::Uuid;

use crate::{
    dao::category::CategoryDAO,
    model::{
        req::category::{CreateCategory, GetCategoryList, UpdateCategoryQuery},
        resp::{category::CategoryVO, Pagination},
    },
    result::AppResult,
};

pub struct CategoryService;
impl CategoryService {
    pub async fn create(req: CreateCategory) -> AppResult<Uuid> {
        CategoryDAO::create(req).await
    }
    pub async fn list(req: GetCategoryList) -> AppResult<Pagination<CategoryVO>> {
        CategoryDAO::list(req).await
    }
    pub async fn delete(ids: Vec<Uuid>) -> AppResult<u64> {
        CategoryDAO::detete(ids).await
    }
    pub async fn update(req: UpdateCategoryQuery) -> AppResult<Uuid> {
        CategoryDAO::update(req).await
    }
}
