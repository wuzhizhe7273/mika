use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct RegisterUser{
    #[garde(length(min=1))]
    pub username:String,
    #[garde(email)]
    pub email:String,
    #[garde(length(min=6,max=64))]
    pub password:String,
}

#[derive(Debug,Validate,Deserialize,ToSchema)]
pub struct LoginUser{
    #[garde(email)]
    pub email:String,
    #[garde(length(min=6,max=64))]
    pub password:String,
}