use uuid::Uuid;
use sea_orm::{Set, EntityTrait};
use crate::{model::req::menu::CreateMenu, result::AppResult, init::db};

pub struct MenuDAO;
impl MenuDAO {
    pub async fn create(req:CreateMenu)->AppResult<Uuid>{
        let menu=entity::menu::ActiveModel{
            id:Set(Uuid::new_v4()),
            name:Set(req.name),
            menu_type:Set(req.menu_type),
            desc:Set(req.desc),
            parent_id:Set(req.parent_id),
            order:Set(req.order),
            ..Default::default()
        };
        let id=entity::menu::Entity::insert(menu).exec(db()).await?.last_insert_id;
        Ok(id)
    }
}