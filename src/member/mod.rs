extern crate rand;

use self::rand::Rng;
use super::SYMBOLS;

#[derive(Clone)]
pub struct Member {
    pub s: String,
    pub score: u32,
}

fn mutate_string(s: &str) -> String {
    //индекс мутируемого символа
    let r_number = rand::thread_rng().gen_range(0, s.len());

    //индекс нового символа
    let r_index = rand::thread_rng().gen_range(0, SYMBOLS.len());
    //новый символ
    let new_char = SYMBOLS.get(r_index).unwrap();

    let mut chars = String::with_capacity(s.len());

    for (i, c) in s.chars().enumerate() {
        chars.push(if i == r_number { *new_char } else { c })
    }

    chars
}

fn get_score(s: &str, expected: &str) -> u32 {
    if s.len() != expected.len() { panic!("bad length"); }

    let mut res = 0;

    for (i, c) in s.chars().enumerate() {
        if c == expected.chars().nth(i).unwrap() { res += 1; }
    }

    res
}

impl Member {
    pub fn new(code: &str, expected: &str) -> Member {
        let new_s = mutate_string(code);

        Member {
            s: new_s.clone(),
            score: get_score(&new_s, expected)
        }
    }

    pub fn get_child(&self, mother: &Member, expected: &str) -> Member {
        if self.s.len() != mother.s.len() {
            panic!("{} != {}", self.s, mother.s);
        }

        let mut child_s = String::with_capacity(self.s.len());

        for i in 0..self.s.len() {
            match rand::thread_rng().gen_range(0, 2) {
                0 => child_s.push(self.s.chars().nth(i).unwrap()),
                _ => child_s.push(mother.s.chars().nth(i).unwrap())
            }
        }

        Member::new(&child_s, expected)
    }
}

#[test]
fn mutate_string_tests_not_same() {
    assert_ne!("Hello", mutate_string("Hello"));
}

#[test]
fn mutate_string_tests_eq_len0() {
    assert_eq!(5, mutate_string("Hello").len())
}

#[test]
fn mutate_string_tests_eq_len1() {
    assert_eq!(15, mutate_string("hellohellohello").len())
}

#[test]
fn get_score_tests() {
    let mut text = String::new();

    for c in SYMBOLS.iter() {
        text.push(*c);
    }
    
    assert_eq!(SYMBOLS.len() as u32, get_score(&text, "world"))
}

#[test]
fn get_new_member_tests() {
    let member = Member::new("world", "world");

    assert_eq!(member.s.len(), 5)
}