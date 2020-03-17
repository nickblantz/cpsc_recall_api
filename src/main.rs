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

// #[derive(Deserialize)]
// struct Form {
//     active: bool
// }

// async fn heavy_work() {
//   let sleep_dur = std::time::Duration::from_secs(5);
//   std::thread::sleep(sleep_dur);
//   println!("Completed some very difficult work taking {} seconds", sleep_dur.as_secs());
// }

// #[get("/")]
// async fn index<T: r2d2::ManageConnection>(data: web::Data<SharedState<'_, T>>) -> impl Responder {
//   let mut index_hit_count = data.index_hit_count.lock().unwrap();
//   *index_hit_count += 1;
//   HttpResponse::Ok().body(format!("hello & welcome to {}\nhit count: {}", data.app_name, index_hit_count))
// }

// #[get("/form")]
// async fn again(info: web::Query<Form>) -> impl Responder {
//   if info.active {
//     HttpResponse::Ok().body("you put in true!!!")
//   } else {
//     HttpResponse::Ok().body("false is not okay")
//   }
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: "localhost".into(),
            port: Some(27017),
        }])
        .max_pool_size(8)
        .build();

    // comment!
    if let Ok(db_client) = Client::with_options(options) {
        let state = web::Data::new(state::AppState {
            db_client: Arc::new(db_client),
        });
        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .service(handlers::list_recalls)
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await?
    } else {
        panic!("Could not connect to the configured database")
    }

    Ok(())
}
