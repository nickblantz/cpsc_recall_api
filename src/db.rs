use crate::entities::Recall;

use actix_web::{web, Error as AWError};
use r2d2;
use r2d2_mongodb;
use serde::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};

pub type Pool = r2d2::Pool<r2d2_mongodb::MongodbConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_mongodb::MongodbConnectionManager>;
pub type Error = std::result::Result<Vec<Recall>, ()>;

pub enum Query {
  GET_ALL_RECALLS,
  GET_RANGE_RECALLS(u32, u32),
}

pub async fn execute(pool: &Pool, query: Query) -> Vec<Recall> {
  let pool = pool.clone();
  web::block( move || {
    if let Ok(conn) = pool.get() {
      match query {
        GET_ALL_RECALLS => get_all_recalls(conn),
        _ => {}
      }
    } else {
      vec![]
    }
  });
  vec![]
}

fn get_all_recalls(conn: Connection) -> Error {

}