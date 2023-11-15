use config::{Config, File};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG:Lazy<AppConfig>=Lazy::new(||{
    AppConfig::new()
});

#[derive(Debug,Deserialize,Default)]
 pub struct AppConfig{
    pub database_url:String,
    pub front_api_addr:String,
    pub admin_api_addr:String,
    pub forestage_addr:String,
    pub backstage_addr:String,
    pub jwt_key:String
}

impl AppConfig {
    fn new()->Self{
        let conf=Config::builder().add_source(File::with_name("config.toml")).build().expect("配置初始化失败");
        let conf:Self=conf.try_deserialize().expect("反序列化配置失败");
        conf
    }
}