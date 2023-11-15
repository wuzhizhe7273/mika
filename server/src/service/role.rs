use uuid::Uuid;

use crate::{result::AppResult, model::req::role::CreateRole, dao::role::RoleDAO};

pub struct RoleService;
impl RoleService {
    pub async fn create(req:CreateRole)->AppResult<Uuid>{
        RoleDAO::create(req).await
    }
}