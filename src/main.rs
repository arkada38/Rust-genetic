extern crate genetic;

use std::io;

fn main() {
//    println!("Hello in Japanese: {}", genetic::hello());
    println!("Пожалуйста, введите предложение.");

    let mut expected = String::new();

    io::stdin().read_line(&mut expected)
        .expect("Не удалось прочитать строку");

    genetic::init_genetic(expected.trim().to_string());
}
