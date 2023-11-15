use sea_orm::{EntityTrait, Set};
use uuid::Uuid;

use crate::{init::db, model::req::role::CreateRole, result::AppResult};

pub struct RoleDAO;
impl RoleDAO {
    pub async fn create(req: CreateRole) -> AppResult<Uuid> {
        let role = entity::role::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
            desc: Set(req.desc),
            parent_id:Set(req.parent),
            ..Default::default()
        };
        let res = entity::role::Entity::insert(role)
            .exec(db())
            .await?
            .last_insert_id;
        if let Some(resources) = req.resource {
            let relations = resources
                .into_iter()
                .map(|r| entity::r_role_rersource::ActiveModel {
                    role_id: Set(res),
                    resource_id: Set(r),
                })
                .collect::<Vec<entity::r_role_rersource::ActiveModel>>();
            let _ =entity::r_role_rersource::Entity::insert_many(relations).exec(db()).await?;
        }
        Ok(res)
    }
}
