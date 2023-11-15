use sea_orm::DatabaseConnection;
use std::cell::OnceCell;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};
pub static mut DB:OnceCell<DatabaseConnection>=OnceCell::new();
pub fn db()->&'static DatabaseConnection{
    unsafe { DB.get().expect("get database connection failed") }
}
pub async fn init() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let database_url = "postgres://wanderer:chjh605319@127.0.0.1:5432/mika";
    let connection=sea_orm::Database::connect(database_url).await.unwrap();
    unsafe { DB.set(connection).expect("Initialize Database Connection Failed")};
    db().ping().await.expect("test database connection failed");
}
