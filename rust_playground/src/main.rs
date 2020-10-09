mod backtrace;
mod dp;
mod leetcode;
mod unionfind;

use elasticsearch::{Elasticsearch, Error, SearchParts};
use mongodb::{
    bson::{doc, Bson},
    options::ClientOptions,
    options::FindOptions,
    Client,
};
use serde_json::{json, Value};

async fn mongo_test() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a collection in the database.
    let db = client.database("local");
    let collection = db.collection("rust-test");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    Ok(())
}

#[tokio::main]
async fn es_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Elasticsearch::default();

    // make a search API call
    let search_response = client
        .search(SearchParts::None)
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .allow_no_indices(true)
        .send()
        .await?;

    // get the HTTP response status code
    let status_code = search_response.status_code();

    // read the response body. Consumes search_response
    let response_body = search_response.json::<Value>().await?;

    // read fields from the response body
    let took = response_body["took"].as_i64().unwrap();

    println!("{:?}", took);

    Ok(())
}

fn main() {
    let a = 0.1;
    let b = 0.2;
    mongo_test();
    es_test();
    println!("{:?}", a + b)
}
