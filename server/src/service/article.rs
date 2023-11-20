use uuid::Uuid;

use crate::{model::{req::article::{CreateArticleQuery, GetArticleList, UpdateArticleQuery}, resp::{Pagination, article::ArticleVO}}, result::AppResult, dao::article::ArticleDAO};

pub struct ArticleService;
impl ArticleService {
    pub async fn create(req:CreateArticleQuery)->AppResult<i64>{
        ArticleDAO::create(req).await
    }
    pub async fn list(req:GetArticleList)->AppResult<Pagination<ArticleVO>>{
        ArticleDAO::list(req).await
    }
    pub async fn delete(ids:Vec<Uuid>)->AppResult<u64>{
        ArticleDAO::delete(ids).await
    }
    pub async fn update(req:UpdateArticleQuery)->AppResult<i64>{
        ArticleDAO::update(req).await
    }
}