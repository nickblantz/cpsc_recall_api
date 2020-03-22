pub mod db;
pub mod entities;
pub mod handlers;
pub mod state;

use {
    actix_web::{web, App, HttpServer},
    mongodb::{
        options::{ClientOptions, StreamAddress},
        Client,
    },
    std::sync::Arc,
};

const HOSTNAME: &str = "localhost";
const PORT: u16 = 27017;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: String::from(HOSTNAME),
            port: Some(PORT),
        }])
        .max_pool_size(8)
        .build();

    if let Ok(db_client) = Client::with_options(options) {
        let state = web::Data::new(state::AppState {
            db_client: Arc::new(db_client),
        });
        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .service(handlers::get_recalls_all)
                .service(handlers::get_recalls_single)
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await?
    } else {
        panic!("Could not connect to the configured database")
    }

    Ok(())
}
