extern crate genetic;
extern crate clap;

use genetic::config;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Genetic")
        .version("1.0")
        .author("Arkadiy K. <arkada38@gmail.com>")
        .about("Evolution of the string")
        .arg(Arg::with_name("STRING")
            .required(true)
            .help("Expected string")
            .index(1))
        .arg(Arg::with_name("population")
            .short("p")
            .value_name("POPULATION")
            .takes_value(true)
            .help("Sets the population"))
        .arg(Arg::with_name("alpha")
            .short("a")
            .value_name("ALPHA")
            .takes_value(true)
            .help("Sets the alpha population"))
        .arg(Arg::with_name("beta")
            .short("b")
            .value_name("BETA")
            .takes_value(true)
            .help("Sets the beta population"))
        .arg(Arg::with_name("output")
            .short("o")
            .help("Prints the output"))
        .get_matches();

    let expected = matches.value_of("STRING").unwrap();

    let population = match matches.value_of("population") {
        Some(x) => x.parse::<u32>().unwrap(),
        None => 100
    };

    let alpha_population = match matches.value_of("alpha") {
        Some(x) => x.parse::<u32>().unwrap(),
        None => 10
    };

    let beta_population = match matches.value_of("beta") {
        Some(x) => x.parse::<u32>().unwrap(),
        None => 50
    };

    let print_output = matches.is_present("output");

    let config = config::ConfigBuilder::new(expected.trim().to_string())
        .population(population)
        .alpha_population(alpha_population)
        .beta_population(beta_population)
        .print_output(print_output)
        .finalize();

    genetic::init_genetic(config);
}
