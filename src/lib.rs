use std::str::FromStr;

use parser::{Ast, Parser};

use crate::tokenizer::Tokenizer;

pub mod parser;
pub mod span;
mod tokenizer;

pub type Json = Ast;

impl FromStr for Json {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Json::parse(s)
  }
}

impl Json {
  pub fn parse(input: &str) -> Result<Json, String> {
    let tokens = Tokenizer::new(input).tokenize()?;
    Parser::new(&tokens).parse()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let json = Json::parse("[{ \"hello\": [\"world\", 1, null, true, { \"a\": [] }] }]").unwrap();

    assert!(matches!(json, Json::Array(_)))
  }

  #[test]
  fn test_from_str() {
    let json = "[{ \"hello\": [\"world\", 1, null, true, { \"a\": [] }] }]"
      .parse::<Json>()
      .unwrap();

    assert!(matches!(json, Json::Array(_)))
  }
}
