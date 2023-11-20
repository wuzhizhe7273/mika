use chrono::{Local, Utc};
use sea_orm::{
    ColumnTrait, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter, QuerySelect, Set,
};
use sea_query::JoinType;
use uuid::Uuid;

use crate::{
    init::db,
    model::{
        req::article::{CreateArticleQuery, GetArticleList, UpdateArticle, UpdateArticleQuery},
        resp::{
            article::ArticleVO,
            category::CategoryOptionVO,
            tag::{TagOptionVO, TagVO},
            user::UserOptionVO,
            Pagination,
        },
    },
    result::{AppError, AppResult},
};

pub struct ArticleDAO;
impl ArticleDAO {
    pub async fn create(req: CreateArticleQuery) -> AppResult<i64> {
        let article = entity::article::ActiveModel {
            user_id: Set(req.user_id),
            title: Set(req.title),
            category_id: Set(req.category),
            content: Set(req.content),
            desc: Set(req.desc),
            ..Default::default()
        };
        let article_id = entity::article::Entity::insert(article)
            .exec(db())
            .await?
            .last_insert_id;
        match req.tags {
            Some(tags) => {
                let tags = tags
                    .into_iter()
                    .map(|t| entity::r_article_tag::ActiveModel {
                        article_id: Set(article_id),
                        tag_id: Set(t),
                    })
                    .collect::<Vec<entity::r_article_tag::ActiveModel>>();
                entity::r_article_tag::Entity::insert_many(tags)
                    .exec(db())
                    .await?;
            }
            None => {}
        }
        Ok(article_id)
    }

    pub async fn list(req: GetArticleList) -> AppResult<Pagination<ArticleVO>> {
        // title和desc的删选条件
        let pages = entity::article::Entity::find()
            .filter(entity::article::Column::Title.contains(req.title))
            .filter(entity::article::Column::Desc.contains(req.desc));
        // category的筛选条件
        let pages = if req.category.len() > 0 {
            pages.filter(entity::article::Column::CategoryId.is_in(req.category))
        } else {
            pages
        };
        // tag的筛选条件
        let pages = if req.tag.len() > 0 {
            pages
                .left_join(entity::r_article_tag::Entity)
                .group_by(entity::article::Column::Id)
                .having(entity::r_article_tag::Column::TagId.is_in(req.tag))
        } else {
            pages
        };
        //获取分页
        let pages = pages.paginate(db(), req.page_size);
        let total = pages.num_items().await?;
        // 获取文章
        let articles = pages.fetch_page(req.page - 1).await?;
        // 获取用户
        let users = articles.load_one(entity::user::Entity, db()).await?;
        // 获取分类
        let categories = articles.load_one(entity::category::Entity, db()).await?;
        // 获取标签
        let tags = articles
            .load_many_to_many(entity::tag::Entity, entity::r_article_tag::Entity, db())
            .await?;
        // 组装结果
        let mut result: Vec<ArticleVO> = Vec::new();
        for i in 0..articles.len() {
            let user = match users[i].clone() {
                Some(u) => Some(UserOptionVO {
                    id: u.id,
                    username: u.username,
                }),
                None => None,
            };
            let category = match categories[i].clone() {
                Some(c) => Some(CategoryOptionVO {
                    id: c.id,
                    name: c.name,
                }),
                None => None,
            };
            let article_tags = tags[i]
                .clone()
                .into_iter()
                .map(|t| TagOptionVO {
                    id: t.id,
                    name: t.name,
                })
                .collect::<Vec<TagOptionVO>>();
            result.push(ArticleVO {
                id: articles[i].id,
                title: articles[i].title.clone(),
                user: user,
                category: category,
                tags: Some(article_tags),
                desc: articles[i].desc.clone(),
                content: Some(articles[i].content.clone()),
                updated_at: articles[i].updated_at,
                created_at: articles[i].created_at,
            })
        }
        Ok(Pagination {
            total: total,
            page: req.page,
            data: result,
        })
    }
    pub async fn delete(ids: Vec<Uuid>) -> AppResult<u64> {
        let rows = entity::article::Entity::delete_many()
            .filter(entity::article::Column::Id.is_in(ids))
            .exec(db())
            .await?
            .rows_affected;
        Ok(rows)
    }
    pub async fn update(req: UpdateArticleQuery) -> AppResult<i64> {
        let mut article: entity::article::ActiveModel = entity::article::Entity::find_by_id(req.id)
            .one(db())
            .await?
            .ok_or(AppError::NotFound)?
            .into();
        article.category_id = Set(req.category);
        article.content = Set(req.content);
        article.title = Set(req.title);
        article.desc = Set(req.desc);
        article.updated_at = Set(Utc::now().into());

        let id = entity::article::Entity::insert(article)
            .exec(db())
            .await?
            .last_insert_id;
        entity::r_article_tag::Entity::delete_many()
            .filter(entity::r_article_tag::Column::ArticleId.eq(id))
            .exec(db())
            .await?;
        let tags = req
            .tags
            .into_iter()
            .map(|t| entity::r_article_tag::ActiveModel {
                article_id: Set(id),
                tag_id: Set(t),
            })
            .collect::<Vec<entity::r_article_tag::ActiveModel>>();
        entity::r_article_tag::Entity::insert_many(tags)
            .exec(db())
            .await?;
        Ok(id)
    }
}
