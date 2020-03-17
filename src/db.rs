use {
    bson::{bson, doc, Document},
    mongodb::{
        error::Error,
        options::FindOptions,
        Client,
        Cursor
    },
    std::result::Result,

    crate::entities::Recall
};

pub enum Query {
    GetRecallsAll,
    GetRecallsFiltered(bson::ordered::OrderedDocument),
    GetRecallsRanged(u32, u32),
}

pub async fn execute(client: &Client, query: Query) -> Cursor {
    match query {
        Query::GetRecallsAll => get_recalls(client),
        _ => get_recalls(client),
    }
}

fn get_recalls(client: &Client) -> Cursor {
  let filter = doc! { "RecallID": { "$gte": 5000 } };
    client.database("recall_db")
      .collection("recalls")
      .find(
        Some(filter), 
        None
      )
      .unwrap() 
      // .collect()
}

// fn testing(client: &Client) {
//   let db = client.database("recall_db");
//   let collection = db.collection("recalls");
//   let filter = doc! { "author": "George Orwell" };
//   let cursor = collection.find(None, None);

// }
