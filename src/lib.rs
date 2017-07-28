extern crate rand;

use self::rand::Rng;

pub mod member;

pub const SYMBOLS: &'static [char] = &[
    ' ', ',', '.', '!', '?', '-', '*', '/', '\\', '|', '=', ';', ':', '"', '\'', '[', ']', '{', '}', '@', '#', '%', '(', ')',
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    //'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    //'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я'
];

pub fn init_genetic(expected: String) {
    println!("Ожидаемая строка: {}:{}", expected, expected.len());

    let population = get_population(&expected);

    let steps = start_genetic(expected, population);
    println!("Задача решена за {} поколений", steps);
}

fn get_population(expected: &str) -> Vec<member::Member> {
    let mut m = 0;
    let mut population = Vec::with_capacity(100);

    loop {
        let new_string = get_random_string(expected.len());
        population.push(member::Member::new(&new_string, expected));

        m += 1;
        if m == 100 { break; }
    }

    population
}

fn get_random_string(n: usize) -> String {
    let mut m = 0;
    let mut res = String::with_capacity(n);

    loop {
        let r_index = rand::thread_rng().gen_range(0, SYMBOLS.len());
        let new_char = SYMBOLS.get(r_index).unwrap();
        res.push(*new_char);

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

        println!("{} - {}:{} of {}", step, pop[0].s, pop[0].score, pop[0].s.len());
        step += 1;

        if pop[0].s == expected { break; }

        pop = get_next_population(pop, &expected);
    }

    step
}

fn get_next_population(population: Vec<member::Member>, expected: &str) -> Vec<member::Member> {
    let mut pop = Vec::with_capacity(100);

    //лучшие десять
    for i in 0..10 {
        //оставляют по десять детей
        let mut childs = 0;
        while childs < 10 {
            //от лучших пятидесяти
            let mother_index = rand::thread_rng().gen_range(0, 50);
            let mother = &population[mother_index];
            pop.push(population[i].get_child(&mother, expected));
            childs += 1;
        }
    }

    pop
}

#[test]
fn get_random_string_tests_eq_len0() {
    assert_eq!(5, get_random_string(5).len())
}

#[test]
fn get_random_string_tests_eq_len1() {
    assert_eq!(15, get_random_string(15).len())
}