use std::error::Error;

use migration::{Migrator, MigratorTrait};
use server::{app::App, init::{init, db}};

#[tokio::main]
async fn main(){
    init().await;
    Migrator::up(db(), None).await.expect("Database migration failed");
    let app=App::new().await;
    let _=app.run().await;
}
