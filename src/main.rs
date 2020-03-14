extern crate r2d2_mongodb;
extern crate r2d2;
extern crate serde;

pub mod db;
pub mod entities;


use {
  actix_web::{ get, web, App, HttpResponse, HttpServer, Responder },
  serde::{ Deserialize },
  std::sync::{ Arc, Mutex },
  r2d2:: { Pool },
  r2d2_mongodb::{ ConnectionOptions, MongodbConnectionManager }
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
  let manager = MongodbConnectionManager::new(
    ConnectionOptions::builder()
      .with_host("localhost", 27017)
      .with_db("recall_db")
      .with_auth("root", "pass")
      .build()
  );

  if let Ok(pool) = Pool::builder().max_size(8).build(manager) {
    HttpServer::new(move || {
      App::new()
        // .service(index)
        // .service(again)
        .app_data(pool.clone())
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await?
  } else {
    panic!("Could not connect to the configured database")
  }

  


  Ok(())
}