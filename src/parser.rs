use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{
  span::{Loc, Span},
  tokenizer::Token,
};

lazy_static! {
  static ref ESCAPES: HashMap<char, &'static str> = HashMap::from([
    ('b', r"\b"),
    ('f', r"\f"),
    ('n', r"\n"),
    ('r', r"\r"),
    ('t', r"\t"),
  ]);
}

#[derive(Debug, PartialEq)]
pub enum Ast {
  String(StringAst),
  Number(NumberAst),
  Boolean(BoolAst),
  Null(NullAst),
  Object(ObjectAst),
  Property(PropertyAst),
  Identifier(IdentifierAst),
  Array(ArrayAst),
}

impl Ast {
  pub fn get_span(&self) -> &Span {
    match self {
      Ast::String(ast) => &ast.span,
      Ast::Number(ast) => &ast.span,
      Ast::Boolean(ast) => &ast.span,
      Ast::Null(ast) => &ast.span,
      Ast::Object(ast) => &ast.span,
      Ast::Property(ast) => &ast.span,
      Ast::Identifier(ast) => &ast.span,
      Ast::Array(ast) => &ast.span,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct StringAst {
  value: String,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NumberAst {
  value: f64,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct BoolAst {
  value: bool,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NullAst {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ObjectAst {
  value: Vec<PropertyAst>,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct PropertyAst {
  key: IdentifierAst,
  value: Box<Ast>,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierAst {
  value: StringAst,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ArrayAst {
  value: Vec<Box<Ast>>,
  span: Span,
}

enum ObjectState {
  Start,
  LeftBrace,
  Property,
  Comma,
}

enum PropertyState {
  Start,
  Key,
  Colon,
}

enum ArrayState {
  Start,
  LeftBracket,
  Value,
  Comma,
}

pub(crate) struct Parser<'a> {
  tokens: &'a [Token],
  len: usize,
  index: usize,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: &'a [Token]) -> Self {
    Self {
      tokens,
      len: tokens.len(),
      index: 0,
    }
  }

  pub fn parse(&mut self) -> Result<Ast, String> {
    if self.len == 0 {
      return self.error_eof();
    }

    self.parse_value()
  }

  fn error_eof(&self) -> Result<Ast, String> {
    Err(format!("Unexpected end of input"))
  }

  fn error_token(&self, token: &Token) -> Result<Ast, String> {
    Err(format!("Unexpected token: {:#?}", token))
  }

  fn create_span(&self, start_span: Option<&Span>, end_span: &Span) -> Span {
    if let Some(start_span) = start_span {
      return Span {
        start: Loc {
          line: start_span.start.line,
          column: start_span.start.column,
          offset: start_span.start.offset,
        },
        end: Loc {
          line: end_span.end.line,
          column: end_span.end.column,
          offset: end_span.end.offset,
        },
      };
    } else {
      return end_span.clone();
    }
  }

  // literal, object, array
  fn parse_value(&mut self) -> Result<Ast, String> {
    self
      .parse_literal()
      .or_else(|_| self.parse_object())
      .or_else(|_| self.parse_array())
  }

  // string, number, boolean, null
  fn parse_literal(&mut self) -> Result<Ast, String> {
    let token = self.tokens.get(self.index).unwrap();

    match token {
      Token::String(token) => {
        let ret = parse_string(&token.value)?;
        self.index += 1;
        Ok(Ast::String(StringAst {
          value: ret,
          span: self.create_span(None, &token.span),
        }))
      }
      Token::Number(token) => {
        self.index += 1;
        Ok(Ast::Number(NumberAst {
          value: token.value,
          span: self.create_span(None, &token.span),
        }))
      }
      Token::Boolean(token) => {
        self.index += 1;
        Ok(Ast::Boolean(BoolAst {
          value: token.value,
          span: self.create_span(None, &token.span),
        }))
      }
      Token::Null(token) => {
        self.index += 1;
        Ok(Ast::Null(NullAst {
          span: self.create_span(None, &token.span),
        }))
      }
      _ => Err(format!("Unexpected token: {:#?}", token)),
    }
  }

  fn parse_object(&mut self) -> Result<Ast, String> {
    let mut state = ObjectState::Start;
    let mut start_span: Option<&Span> = None;

    let mut object_ast = ObjectAst {
      value: vec![],
      span: Span::default(),
    };

    while let Some(token) = self.tokens.get(self.index) {
      match state {
        ObjectState::Start => match token {
          Token::LeftBrace(token) => {
            state = ObjectState::LeftBrace;
            start_span = Some(&token.span);
            self.index += 1;
          }
          _ => {
            return self.error_token(token);
          }
        },
        ObjectState::LeftBrace => match token {
          Token::RightBrace(token) => {
            self.index += 1;
            object_ast.span = self.create_span(start_span, &token.span);
            return Ok(Ast::Object(object_ast));
          }
          _ => {
            if let Ast::Property(property) = self.parse_property()? {
              object_ast.value.push(property);
              state = ObjectState::Property;
            } else {
              return self.error_token(token);
            }
          }
        },
        ObjectState::Property => match token {
          Token::Comma(_) => {
            state = ObjectState::Comma;
            self.index += 1;
          }
          Token::RightBrace(token) => {
            self.index += 1;
            object_ast.span = self.create_span(start_span, &token.span);
            return Ok(Ast::Object(object_ast));
          }
          _ => return self.error_token(token),
        },
        ObjectState::Comma => {
          if let Ast::Property(property) = self.parse_property()? {
            object_ast.value.push(property);
            state = ObjectState::Property;
          } else {
            return self.error_token(token);
          }
        }
      }
    }

    self.error_eof()
  }

  fn parse_property(&mut self) -> Result<Ast, String> {
    let mut state = PropertyState::Start;
    let mut start_span: Option<&Span> = None;
    let mut identifier: Option<IdentifierAst> = None;

    while let Some(token) = self.tokens.get(self.index) {
      match state {
        PropertyState::Start => match token {
          Token::String(token) => {
            start_span = Some(&token.span);
            identifier = Some(IdentifierAst {
              value: StringAst {
                value: parse_string(&token.value)?,
                span: token.span.clone(),
              },
              span: token.span.clone(),
            });
            state = PropertyState::Key;
            self.index += 1;
          }
          _ => return self.error_token(token),
        },
        PropertyState::Key => match token {
          Token::Colon(_) => {
            state = PropertyState::Colon;
            self.index += 1;
          }
          _ => return self.error_token(token),
        },
        PropertyState::Colon => {
          let value = self.parse_value()?;
          let value_span = value.get_span().clone();

          return Ok(Ast::Property(PropertyAst {
            key: identifier.unwrap(),
            value: Box::new(value),
            span: self.create_span(start_span, &value_span),
          }));
        }
      }
    }

    self.error_eof()
  }

  fn parse_array(&mut self) -> Result<Ast, String> {
    let mut state = ArrayState::Start;
    let mut start_span: Option<&Span> = None;
    let mut array_value = vec![];

    while let Some(token) = self.tokens.get(self.index) {
      match state {
        ArrayState::Start => match token {
          Token::LeftBracket(token) => {
            start_span = Some(&token.span);
            state = ArrayState::LeftBracket;
            self.index += 1;
          }
          _ => return self.error_token(token),
        },
        ArrayState::LeftBracket => match token {
          Token::RightBracket(token) => {
            self.index += 1;
            return Ok(Ast::Array(ArrayAst {
              value: array_value,
              span: self.create_span(start_span, &token.span),
            }));
          }
          _ => {
            let value = self.parse_value()?;
            array_value.push(Box::new(value));
            state = ArrayState::Value;
          }
        },
        ArrayState::Value => match token {
          Token::RightBracket(token) => {
            self.index += 1;
            return Ok(Ast::Array(ArrayAst {
              value: array_value,
              span: self.create_span(start_span, &token.span),
            }));
          }
          Token::Comma(_) => {
            state = ArrayState::Comma;
            self.index += 1;
          }
          _ => return self.error_token(token),
        },
        ArrayState::Comma => {
          let value = self.parse_value()?;
          array_value.push(Box::new(value));
          state = ArrayState::Value;
        }
      }
    }

    self.error_eof()
  }
}

fn parse_string(quoted_input: &str) -> Result<String, String> {
  let mut ret = String::new();

  // 去除首尾双引号
  let chars = &quoted_input[1..quoted_input.len() - 1]
    .chars()
    .collect::<Vec<char>>();
  let mut index = 0;

  while index < chars.len() {
    let c = chars.get(index).unwrap();
    index += 1;

    match c {
      '\\' => {
        let next_c = chars.get(index).unwrap();
        index += 1;

        match next_c {
          'u' => {
            // 解析 unicode 字符
            let unicode =
              u16::from_str_radix(&chars[index..index + 4].iter().collect::<String>(), 16).unwrap();
            ret.push(char::from_u32(unicode as u32).unwrap());
            index += 4;
          }
          '"' | '\\' | '/' => {
            ret.push(next_c.clone());
          }
          'b' | 'f' | 'n' | 'r' | 't' => {
            ret.push_str(ESCAPES.get(next_c).unwrap());
          }
          _ => return Err(format!("Unexpected escape character: {}", next_c)),
        }
      }
      _ => ret.push(c.clone()),
    }
  }

  Ok(ret)
}
