extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    isMail: bool
}

fn main() {
    let json_str = r#"
    {
     "name":"Domenic",
     "age":65,
     "isMail":true
    }
    "#;

    let result = serde_json::from_str(json_str);

    if result.is_ok() {
        let pojo : Person = result.unwrap();
        println!("the name is {}", pojo.name);
    }else {
        println!("ooops, error");
    }
}
