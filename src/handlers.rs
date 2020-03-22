use {
    crate::{
        db::{execute, Query},
        entities::Recalls,
        state::AppState,
    },
    actix_web::{
        get, post,
        web::{Data, Json, Path},
        Either, HttpResponse,
    },
};

#[get("/recalls")]
pub async fn get_recalls_all(data: Data<AppState>) -> Either<Json<Recalls>, HttpResponse> {
    match execute(&data.db_client.clone(), Query::GetRecallsAll).await {
        Ok(recall) => Either::A(Json(recall)),
        Err(_e) => Either::B(HttpResponse::from("oop")),
    }
}

#[get("/recall/{id}")]
pub async fn get_recalls_single(
    data: Data<AppState>,
    id: Path<i32>,
) -> Either<Json<Recalls>, HttpResponse> {
    match execute(
        &data.db_client.clone(),
        Query::GetRecallsSingle(id.into_inner()),
    )
    .await
    {
        Ok(recall) => Either::A(Json(recall)),
        Err(_e) => Either::B(HttpResponse::from("oop")),
    }
}

// #[post("/recall/{id}")]
// pub async fn post_recall(
//     data: Data<AppState>,
//     id: Path<i32>,
//     payload: Json<Recalls>
// ) -> Either<Json<Recalls>, HttpResponse> {
  
// }


