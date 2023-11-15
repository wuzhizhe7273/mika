use uuid::Uuid;

use crate::{model::req::tag::CreateTag, result::AppResult,dao::tag::TagDAO};

pub struct TagService;
impl TagService {
    pub async fn create(req:CreateTag)->AppResult<Uuid>{
        TagDAO::create(req).await
    }
}