mod solution;

extern crate redis;
extern crate rand;
extern crate elasticsearch;
extern crate serde_json;
extern crate tokio;

use elasticsearch::{Elasticsearch, Error, SearchParts};
use serde_json::{json, Value};
use tokio::prelude::*;
use rand::prelude::*;
use redis::Commands;
use solution::Tests;


fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.get("as1392")
}

fn test_rand() {
 let x: i32 = random();
    println!("{:?}", x);
}

#[tokio::main]
async fn es_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Elasticsearch::default();
    let response = client
    .search(SearchParts::Index(&["zgzcw"]))
    .from(0)
    .size(1)
    .body(json!({
        "query": {
            "match": {
                "hostname": "皇马"
            }
        }
    }))
    .send()
    .await?;

    let response_body = response.json::<Value>().await?;
    let took = response_body["took"].as_i64().unwrap();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        println!("{:?}", hit["_source"]);
    }

    Ok(())
}

fn main() {

    test_rand();
    println!("{:?}", fetch_an_integer());
    es_test();

    Tests::knapsack_test()
}
