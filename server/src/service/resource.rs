use uuid::Uuid;

use crate::{model::req::resource::CreateResource, result::AppResult, dao::resource::ResourceDAO};

pub struct ResourceService;
impl ResourceService {
    pub async fn create(req:CreateResource)->AppResult<Uuid>{
        ResourceDAO::create(req).await
    }
}