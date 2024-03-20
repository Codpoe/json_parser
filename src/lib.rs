use std::str::FromStr;

use parser::{Ast, Parser};

use crate::tokenizer::Tokenizer;

pub mod parser;
pub mod span;
mod tokenizer;
pub mod visit;

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
  use crate::visit::Visit;

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

  #[test]
  fn test_visit() {
    let mut json = "{\"hello\":\"world\"}".parse::<Json>().unwrap();

    struct Visitor {
      pub property_pos: (usize, usize),
      pub merged_string: String,
    }

    impl Visit for Visitor {
      fn visit_property(&mut self, ast: &mut parser::PropertyAst) {
        self.property_pos = (ast.span.start.offset, ast.span.end.offset);

        self.visit_identifier(&mut ast.key);
        self.visit_property_value(&mut ast.value);
      }

      fn visit_string(&mut self, ast: &mut parser::StringAst) {
        if self.merged_string.is_empty() {
          self.merged_string.push_str(&ast.value);
        } else {
          self.merged_string.push_str(&format!("_{}", ast.value));
        }
      }
    }

    let mut visitor = Visitor {
      property_pos: (0, 0),
      merged_string: String::new(),
    };

    visitor.visit_json(&mut json);
    assert_eq!(visitor.property_pos, (1, 16));
    assert_eq!(visitor.merged_string, "hello_world");
  }
}
