use super::ALPHABET;

#[derive(Debug)]
pub struct Config {
  pub expected: String,
  pub population: u32,
  pub alpha_population: u32,
  pub beta_population: u32,
  pub print_output: bool,
}

pub struct ConfigBuilder {
  expected: String,
  population: u32,
  alpha_population: u32,
  beta_population: u32,
  print_output: bool,
}

impl ConfigBuilder {
  pub fn new(expected: String) -> ConfigBuilder {
    ConfigBuilder {
      expected: filter_expected_string(expected),
      population: 100,
      alpha_population: 10,
      beta_population: 50,
      print_output: true,
    }
  }

  pub fn population(&mut self, population: u32) -> &mut ConfigBuilder {
    self.population = population;
    self
  }

  pub fn alpha_population(&mut self, alpha_population: u32) -> &mut ConfigBuilder {
    self.alpha_population = alpha_population;
    self
  }

  pub fn beta_population(&mut self, beta_population: u32) -> &mut ConfigBuilder {
    self.beta_population = beta_population;
    self
  }

  pub fn print_output(&mut self, print_output: bool) -> &mut ConfigBuilder {
    self.print_output = print_output;
    self
  }

  pub fn finalize(&self) -> Config {
    if self.alpha_population > self.beta_population {
      panic!("Alfa population must be less or equal of beta population.")
    }
    if self.beta_population > self.population {
      panic!("Beta population must be less or equal of population.")
    }
    Config {
      expected: self.expected.to_string(),
      population: self.population,
      alpha_population: self.alpha_population,
      beta_population: self.beta_population,
      print_output: self.print_output,
    }
  }
}

fn filter_expected_string(expected: String) -> String {
  let mut res = String::new();
  let mut is_filtered = false;

  for c in expected.chars() {
    match ALPHABET.contains(c) {
      true => res.push(c),
      false => is_filtered = true,
    }
  }

  if is_filtered {
    println!("The expected string was filtered.");
  };

  res
}

#[test]
fn filter_expected_string_test() {
  assert_eq!(
    "hachiko",
    filter_expected_string("hachiko忠犬ハチ公".to_string())
  )
}
