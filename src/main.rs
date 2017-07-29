extern crate genetic;

use std::io;

fn main() {
    println!("Please enter a string");

    let mut expected = String::new();

    io::stdin().read_line(&mut expected)
        .expect("Could not read line");

    genetic::init_genetic(expected.trim().to_string());
}
