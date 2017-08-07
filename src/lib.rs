extern crate rand;

use self::rand::Rng;

pub mod config;
mod member;

pub const ALPHABET: &'static str = &" \
,.!?-*/\\|=;:\"'[]{}@#%()\
1234567890\
abcdefghijklmnopqrstuvwxyz\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ\
абвгдеёжзийклмнопрстуфхцчшщъыьэюя";

pub fn init_genetic(config: config::Config) {
    println!("Expected string: {} with length {}", config.expected, config.expected.chars().count());
    println!("p {} a {} b {} o {}",
             config.population, config.alpha_population,
             config.beta_population, config.print_output);

    let population = get_population(&config);

    let steps = start_genetic(&config, population);
    println!("The problem is solved in {} generations", steps);
}

fn get_population(config: &config::Config) -> Vec<member::Member> {
    let mut m = 0;
    let mut population = Vec::new();

    loop {
        let new_string = get_random_string(config.expected.chars().count());
        population.push(member::Member::new(&new_string, &config.expected));

        m += 1;
        if m == config.population { break population; }
    }
}

fn get_random_string(n: usize) -> String {
    let mut m = 0;
    let mut res = String::with_capacity(n * 4);
    let mut rng = rand::thread_rng();

    loop {
        let r_index = rng.gen_range(0, ALPHABET.chars().count());
        let new_char = ALPHABET.chars().nth(r_index).unwrap();
        res.push(new_char);

        m += 1;
        if m == n { break res; }
    }
}

fn start_genetic(config: &config::Config, population: Vec<member::Member>) -> usize {
    let mut pop = population.clone();
    let mut step = 0;

    loop {
        pop.sort_by(|a, b| b.score.cmp(&a.score));

        if config.print_output {
            let percents = 100.0f32 * pop[0].score as f32 / pop[0].s.chars().count() as f32;
            println!("{number:>width$} - {} ({:.*}%)",
                     pop[0].s,
                     2, percents,
                     number = step + 1,
                     width = 4);
        }

        step += 1;

        if pop[0].s == config.expected { break step; }

        pop = get_next_population(config, pop);
    }
}

fn get_next_population(config: &config::Config, population: Vec<member::Member>) -> Vec<member::Member> {
    let mut pop = Vec::new();
    let mut rng = rand::thread_rng();

    //The top ten
    for i in 0..config.alpha_population {
        //Leave ten children each
        let mut childs = 0;
        while childs < 10 {
            //From the best fifty
            let mother_index = rng.gen_range(0, config.beta_population);
            let mother = &population[mother_index as usize];
            pop.push(population[i as usize].get_child(&mother, &config.expected));
            childs += 1;
        }
    }

    pop
}

#[test]
fn get_random_string_test_eq_len0() {
    assert_eq!(5, get_random_string(5).chars().count())
}

#[test]
fn get_random_string_test_eq_len1() {
    assert_eq!(15, get_random_string(15).chars().count())
}