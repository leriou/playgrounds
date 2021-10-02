// extern crate redis;

use redis::Commands;

pub fn fetch_value() -> redis::RedisResult<String> {
    
   let client = redis::Client::open("redis://127.0.0.1:6379")?;

   let mut con = client.get_connection()?;

   let _:() = con.sadd("r_s", "sss1")?;

   con.get("rust_set")
}