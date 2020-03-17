use {
    actix_web::{
        get,
        web::{self, Json},
        App, HttpResponse, HttpServer, Responder, Result,
    },
    mongodb::{
        options::{ClientOptions, StreamAddress},
        Client,
    },

    crate::{
      db::{execute, Query},
      entities::Remedy,
      state::AppState,
    },
};

#[get("/")]
pub async fn list_recalls(data: web::Data<AppState>) -> Result<Json<Remedy>> {
    let client = &data.db_client.clone();

    for recall in execute(client, Query::GetRecallsAll).await {
        match recall {
            Ok(recall) => println!("{:?}", recall),
            Err(_e) => {}
        }
    }

    Ok(Json(Remedy {
        name: Some(String::from("dont do it again")),
    }))
}
