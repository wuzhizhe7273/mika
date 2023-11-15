use sea_orm::{Set, EntityTrait};
use uuid::Uuid;

use crate::{model::req::article::CreateArticleQuery, result::AppResult, init::db};

pub struct ArticleDAO;
impl ArticleDAO {
    pub async fn create(req:CreateArticleQuery)->AppResult<Uuid>{
        let article=entity::article::ActiveModel{
            id:Set(Uuid::new_v4()),
            user_id:Set(req.user_id),
            title:Set(req.title),
            category_id:Set(req.category),
            content:Set(req.content),
            desc:Set(req.desc),
            ..Default::default()
        };
        let article_id=entity::article::Entity::insert(article).exec(db()).await?.last_insert_id;
        match req.tags {
            Some(tags) => {
                let tags=tags.into_iter().map(|t|{entity::r_article_tag::ActiveModel{
                    article_id:Set(article_id),
                    tag_id:Set(t)
                }}).collect::<Vec<entity::r_article_tag::ActiveModel>>();
                entity::r_article_tag::Entity::insert_many(tags).exec(db()).await?;
            },
            None =>{},
        }
        Ok(article_id)
    }
}