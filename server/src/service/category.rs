use uuid::Uuid;

use crate::{
    dao::category::CategoryDAO,
    model::{
        req::{category::{CreateCategory, GetCategoryList, UpdateCategoryQuery}, PermQuery},
        resp::{category::CategoryVO, Pagination},
    },
    result::AppResult,
};

pub struct CategoryService;
impl CategoryService {
    pub async fn create(req: PermQuery<CreateCategory>) -> AppResult<i64> {
        CategoryDAO::create(req).await
    }
    pub async fn list(req: GetCategoryList) -> AppResult<Pagination<CategoryVO>> {
        CategoryDAO::list(req).await
    }
    pub async fn delete(ids: Vec<Uuid>) -> AppResult<u64> {
        CategoryDAO::detete(ids).await
    }
    pub async fn update(req: UpdateCategoryQuery) -> AppResult<i64> {
        CategoryDAO::update(req).await
    }
}
