use ferris_says::say; //using the crate
use std::io::{stdout, BufWriter};

fn main(){
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans! It's the Rust World!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
/*

fn main() {
    println!("Hello, world!");
}

*/