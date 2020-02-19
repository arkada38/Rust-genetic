use super::ALPHABET;
use rand::Rng;

#[derive(Clone)]
pub struct Member {
  pub s: String,
  pub score: u32,
}

fn mutate_string(s: &str) -> String {
  let mut rng = rand::thread_rng();

  //Index of a mutable character
  let r_number = rng.gen_range(0, s.chars().count());

  //New character index
  let r_index = rng.gen_range(0, ALPHABET.chars().count());
  //New character
  let new_char = ALPHABET.chars().nth(r_index).unwrap();

  let mut chars = String::with_capacity(s.len());

  for (i, c) in s.chars().enumerate() {
    chars.push(if i == r_number { new_char } else { c })
  }

  chars
}

fn get_score(s: &str, expected: &str) -> u32 {
  s.chars().enumerate().fold(0, |res, (i, c)| {
    match c == expected.chars().nth(i).unwrap() {
      true => res + 1,
      false => res,
    }
  })
}

impl Member {
  pub fn new(code: &str, expected: &str) -> Member {
    let new_s = mutate_string(code);

    Member {
      s: new_s.clone(),
      score: get_score(&new_s, expected),
    }
  }

  pub fn get_child(&self, partner: &Member, expected: &str) -> Member {
    let mut child_s = String::with_capacity(self.s.len());
    let mut rng = rand::thread_rng();

    for i in 0..self.s.chars().count() {
      match rng.gen::<bool>() {
        true => child_s.push(self.s.chars().nth(i).unwrap()),
        false => child_s.push(partner.s.chars().nth(i).unwrap()),
      }
    }

    Member::new(&child_s, expected)
  }
}

#[test]
fn mutate_string_test_not_same() {
  assert_ne!("Hello", mutate_string("Hello"));
}

#[test]
fn mutate_string_test_eq_len0() {
  assert_eq!(5, mutate_string("Hello").chars().count())
}

#[test]
fn mutate_string_test_eq_len1() {
  assert_eq!(15, mutate_string("hello five four").chars().count())
}

#[test]
fn get_score_test() {
  assert_eq!(1, get_score("dare", "five"))
}

#[test]
fn get_score_test_1() {
  assert_eq!(2, get_score("dare", "wire"))
}

#[test]
fn get_score_test_2() {
  assert_eq!(4, get_score("dare", "dare"))
}

#[test]
fn get_new_member_test() {
  let member = Member::new("world", "world");

  assert_eq!(member.s.chars().count(), 5)
}
