extern crate serde_json;
extern crate serde;
extern crate reqwest;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    isMail: bool
}

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

// fn main() {
//     let json_str = r#"
//     {
//      "name":"Domenic",
//      "age":65,
//      "isMail":true
//     }
//     "#;
//
//     let result = serde_json::from_str(json_str);
//
//     if result.is_ok() {
//         let pojo : Person = result.unwrap();
//         println!("the name is {}", pojo.name);
//     }else {
//         println!("ooops, error");
//     }
// }
