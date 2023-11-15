use crate::{config::CONFIG, router};

pub struct App{

}

impl App{
    pub async fn new()->Self{
        App{

        }
    }
    pub async fn run(self){
        // tracing_subscriber::registry().with(fmt::layer()).init();
        let backrouter=router::backrouter::create().await;
        let admin=tokio::spawn(async{
            axum::Server::bind(&CONFIG.admin_api_addr.parse().unwrap()).serve(backrouter.into_make_service()).await.unwrap();
        });
        // tokio::spawn(async{
        //     axum::Server::bind(&self.addr.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
        // }).await.unwrap();
        tracing::info!("server is runing on http://{}",CONFIG.admin_api_addr);
        admin.await.unwrap();
    }
}