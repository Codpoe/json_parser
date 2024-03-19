use std::collections::HashMap;

mod span;
mod tokenizer;

pub enum Json {
  String(String),
  Number(f64),
  Boolean(bool),
  Null,
  Object(HashMap<String, Box<Json>>),
  Array(Vec<Box<Json>>),
}

impl Json {
  pub fn parse(input: &str) {}

  pub fn stringify(&self) -> String {
    "".to_string()
  }
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
