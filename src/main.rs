extern crate genetic;

//use std::env;
use std::io;
use genetic::config;

fn main() {
//    let args: Vec<String> = env::args().collect();

    let mut expected = String::new();

    println!("Please enter a string");
    io::stdin().read_line(&mut expected)
        .expect("Could not read line");

    let config = config::ConfigBuilder::new(expected.trim().to_string())
        .population(100)
        .alpha_population(10)
        .beta_population(50)
        .print_output(true)
        .finalize();

    genetic::init_genetic(config);
}

//fn print_help() {
//    println!("Prints the latest information.");
//    println!("    -h, --help     : show this information");
//}
