//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::article::Entity")]
    Article,
    #[sea_orm(has_many = "super::r_user_role::Entity")]
    RUserRole,
}

impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

impl Related<super::r_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RUserRole.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::r_user_role::Relation::Role.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::r_user_role::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
