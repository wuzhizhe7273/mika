use uuid::Uuid;

use crate::{
    dao::tag::TagDAO,
    model::{
        req::tag::{CreateTag, GetTagList, UpdateTagQuery},
        resp::{tag::TagVO, Pagination},
    },
    result::AppResult,
};

pub struct TagService;
impl TagService {
    pub async fn create(req: CreateTag) -> AppResult<Uuid> {
        TagDAO::create(req).await
    }
    pub async fn list(req: GetTagList) -> AppResult<Pagination<TagVO>> {
        TagDAO::list(req).await
    }
    pub async fn delete(ids: Vec<Uuid>) -> AppResult<u64> {
        TagDAO::delete(ids).await
    }
    pub async fn update(req:UpdateTagQuery)->AppResult<Uuid>{
        TagDAO::update(req).await
    }
}
