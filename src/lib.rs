extern crate rand;

use self::rand::Rng;

pub mod member;

pub const ALPHABET: &'static &str = &" ,.!?-*/\\|=;:\"'[]{}@#%()\
    1234567890\
    abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ\
    абвгдеёжзийклмнопрстуфхцчшщъыьэюя";

pub fn init_genetic(mut expected: String) {
    expected = filter_expected_string(expected);

    println!("Expected string: {} with length {}", expected, expected.chars().count());

    let population = get_population(&expected);

    let steps = start_genetic(expected, population);
    println!("The problem is solved in {} generations", steps);
}

fn filter_expected_string(expected: String) -> String {
    let mut res = String::new();
    let mut is_filtered = false;

    for c in expected.chars() {
        match ALPHABET.contains(c) {
            true => res.push(c),
            false => is_filtered = true
        }
    }

    if is_filtered { println!("The expected string was filtered."); };

    res
}

fn get_population(expected: &str) -> Vec<member::Member> {
    let mut m = 0;
    let mut population = Vec::with_capacity(100);

    loop {
        let new_string = get_random_string(expected.chars().count());
        population.push(member::Member::new(&new_string, expected));

        m += 1;
        if m == 100 { break; }
    }

    population
}

fn get_random_string(n: usize) -> String {
    let mut m = 0;
    let mut res = String::with_capacity(n * 4);

    loop {
        let r_index = rand::thread_rng().gen_range(0, ALPHABET.chars().count());
        let new_char = ALPHABET.chars().nth(r_index).unwrap();
        res.push(new_char);

        m += 1;
        if m == n { break; }
    }

    res
}

fn start_genetic(expected: String, population: Vec<member::Member>) -> usize {
    let mut step = 0;
    let mut pop = population.clone();

    loop {
        pop.sort_by(|a, b| b.score.cmp(&a.score));

        let percents = 100.0f32 * pop[0].score as f32 / pop[0].s.chars().count() as f32;
        println!("{number:>width$} - {} ({:.*}%)", pop[0].s, 2, percents, number = step + 1, width = 4);
        step += 1;

        if pop[0].s == expected { break; }

        pop = get_next_population(pop, &expected);
    }

    step
}

fn get_next_population(population: Vec<member::Member>, expected: &str) -> Vec<member::Member> {
    let mut pop = Vec::with_capacity(100);

    //The top ten
    for i in 0..10 {
        //Leave ten children each
        let mut childs = 0;
        while childs < 10 {
            //From the best fifty
            let mother_index = rand::thread_rng().gen_range(0, 50);
            let mother = &population[mother_index];
            pop.push(population[i].get_child(&mother, expected));
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

#[test]
fn filter_expected_string_test() {
    assert_eq!("hachiko", filter_expected_string("hachiko忠犬ハチ公".to_string()))
}