use {
    crate::entities::{Recall, Recalls},
    bson::{bson, doc, ordered::OrderedDocument, Bson, DecoderError},
    mongodb::{error::Error, options::FindOptions, Client},
    std::result::Result,
};

#[derive(Debug)]
pub struct QueryError {
    source: Box<dyn std::error::Error + 'static>,
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "couldn't complete the query :(")
    }
}

impl std::error::Error for QueryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

type QueryResult<T> = Result<T, QueryError>;

pub enum Query {
    GetRecallsAll,
    GetRecallsFiltered(bson::ordered::OrderedDocument),
    GetRecallsRanged(i32, i32),
    GetRecallsSingle(i32),
}

pub async fn execute(client: &Client, query: Query) -> QueryResult<Recalls> {
    let result = match query {
        Query::GetRecallsAll => get_recalls(client, None),
        Query::GetRecallsFiltered(filter) => get_recalls(client, Some(filter)),
        Query::GetRecallsRanged(lower, upper) => get_recalls(
            client,
            Some(doc! {
              "RecallID": {
                "$gte": lower,
                "$lt": upper
              }
            }),
        ),
        Query::GetRecallsSingle(id) => get_recalls(
            client,
            Some(doc! {
              "RecallID": {
                "$eq": id
              }
            }),
        ),
    };
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(QueryError {
            source: Box::new(e),
        }),
    }
}

fn get_recalls(
    client: &Client,
    filter: Option<OrderedDocument>,
) -> Result<Recalls, bson::DecoderError> {
    println!("querying for: {:?}", &filter);
    client
        .database("recall_db")
        .collection("recalls")
        .find(filter, None)
        .unwrap()
        .filter_map(|record| match record {
            Ok(r) => Some(bson::from_bson(Bson::Document(r))),
            Err(_) => None,
        })
        .collect::<Result<Recalls, DecoderError>>()
}
