use uuid::Uuid;

use crate::{model::req::article::CreateArticleQuery, result::AppResult, dao::article::ArticleDAO};

pub struct ArticleService;
impl ArticleService {
    pub async fn create(req:CreateArticleQuery)->AppResult<Uuid>{
        ArticleDAO::create(req).await
    }
}