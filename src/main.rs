//use ferris_says::say; //using the crate
use std::{fs, collections::HashMap};

fn main(){
    let data = fs::read_to_string("./sample.json")
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("JSON does not have correct format");

    //dbg!(json);

    dbg!(json.as_object());
}
