extern crate serde_json;
extern crate serde;
extern crate reqwest;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .json::<User>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

