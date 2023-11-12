//use ferris_says::say; //using the crate
use std::fs;
use serde_json;





fn main() {
    //json
    let data = fs::read_to_string("./dy.json")
        .unwrap();

    let _json: serde_json::Value = serde_json::from_str(&data)
        .expect("JSON does not have correct format");

    //dbg!(json);


    //database
    
}
