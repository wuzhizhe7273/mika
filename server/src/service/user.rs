use chrono::{Utc, Duration};
use uuid::Uuid;

use crate::{result::AppResult, model::req::user::{RegisterUser, LoginUser}, util::pwd::{hash_password, verify}, dao::user::UserDAO, extractor::jwt::Claims};
pub struct UserService;

impl UserService {
    pub async fn register(req:RegisterUser)->AppResult<i64>{
        let pwd=hash_password(req.password).await?;
        let user=RegisterUser{
            username:req.username,
            email:req.email,
            password:pwd
        };
        UserDAO::create(user).await
    }
    pub async fn delete(ids:Vec<Uuid>)->AppResult<u64>{
       UserDAO::delete(ids).await
    }
    // 登录
    pub async fn login(req:LoginUser)->AppResult<String>{
        let (id,hashed_password)=UserDAO::get_pwd(req.email).await?;
        let _=verify(req.password, hashed_password).await?;
        let claims=Claims{
            id:id,
            exp:(Utc::now()+Duration::hours(24)).timestamp() as usize
        };
        let token=claims.encode().await?;
        Ok(token)
    }
}