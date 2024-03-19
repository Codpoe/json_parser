use crate::span::{Loc, Span};

#[derive(Debug, PartialEq)]
pub enum Token {
  LeftBrace(LeftBraceToken),
  RightBrace(RightBraceToken),
  LeftBracket(LeftBracketToken),
  RightBracket(RightBracketToken),
  Colon(ColonToken),
  Comma(CommaToken),
  String(StringToken),
  Number(NumberToken),
  Boolean(BoolToken),
  Null(NullToken),
}

#[derive(Debug, PartialEq)]
pub struct LeftBraceToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct RightBraceToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct LeftBracketToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct RightBracketToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ColBracketToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ColonToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct CommaToken {
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct StringToken {
  value: String,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NumberToken {
  value: f64,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct BoolToken {
  value: bool,
  span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NullToken {
  span: Span,
}

enum StringState {
  Start,
  QuoteOrChar,
  Escape,
}

enum NumberState {
  Start,
  Minus,
  Zero,
  Digit,
  Fraction,
  Point,
  Exp,
  ExpSignOrDigit,
}

const TRUE_LEN: usize = "true".len();
const FALSE_LEN: usize = "false".len();
const NULL_LEN: usize = "null".len();

pub struct Tokenizer<'a> {
  input: &'a str,
  chars: Vec<char>,
  len: usize,
  index: usize,
  line: usize,
  column: usize,
}

impl<'a> Tokenizer<'a> {
  pub fn new(input: &'a str) -> Self {
    let chars = input.chars().collect::<Vec<char>>();
    let len = chars.len();

    Self {
      input,
      chars,
      len,
      index: 0,
      line: 1,
      column: 1,
    }
  }

  pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    while self.index < self.len {
      if let Some(_) = self.whitespace() {
        continue;
      }

      let token = self
        .punctuation()
        .or_else(|| self.string())
        .or_else(|| self.number())
        .or_else(|| self.boolean())
        .or_else(|| self.null());

      if let Some(token) = token {
        tokens.push(token);
      } else {
        return Err(format!(
          "Unexpected symbol `` at {:#?}:{:#?}",
          self.line, self.column
        ));
      }
    }

    Ok(tokens)
  }

  fn line_span(&self, size: usize) -> Span {
    Span {
      start: Loc {
        line: self.line,
        column: self.column,
        offset: self.index,
      },
      end: Loc {
        line: self.line,
        column: self.column + size,
        offset: self.index + size,
      },
    }
  }

  fn substring(&self, start: usize, end: usize) -> String {
    self.chars.iter().skip(start).take(end - start).collect()
  }

  fn whitespace(&mut self) -> Option<()> {
    let c = self.chars.get(self.index).unwrap();

    match c {
      ' ' | '\t' => {
        self.index += 1;
        self.column += 1;
        Some(())
      }
      // CR (Unix)
      '\r' => {
        self.index += 1;
        self.line += 1;
        self.column = 1;

        // CRLF (Windows)
        if *c == '\n' {
          self.index += 1;
        }

        Some(())
      }
      // LF (MacOS)
      '\n' => {
        self.index += 1;
        self.line += 1;
        self.column = 1;
        Some(())
      }
      _ => None,
    }
  }

  fn punctuation(&mut self) -> Option<Token> {
    let c = self.chars.get(self.index).unwrap();

    match c {
      '{' => {
        let token = Token::LeftBrace(LeftBraceToken {
          span: self.line_span(1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      '}' => {
        let token = Token::RightBrace(RightBraceToken {
          span: self.line_span(1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      '[' => {
        let token = Token::LeftBracket(LeftBracketToken {
          span: self.line_span(1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      ']' => {
        let token = Token::RightBracket(RightBracketToken {
          span: self.line_span(1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      ':' => {
        let token = Token::Colon(ColonToken {
          span: self.line_span(1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      ',' => {
        let token = Token::Comma(CommaToken {
          span: self.line_span(1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      _ => None,
    }
  }

  fn string(&mut self) -> Option<Token> {
    let mut state = StringState::Start;
    let start_index = self.index;

    while let Some(c) = self.chars.get(self.index) {
      match state {
        StringState::Start => match c {
          // 开始引号
          '"' => {
            state = StringState::QuoteOrChar;
            self.index += 1;
            self.column += 1;
          }
          _ => return None,
        },
        StringState::QuoteOrChar => match c {
          // 结束引号
          '"' => {
            let token = Token::String(StringToken {
              value: self.substring(start_index, self.index + 1),
              span: self.line_span(self.index + 1 - start_index),
            });
            self.index += 1;
            self.column += 1;

            return Some(token);
          }
          // 转义字符
          '\\' => {
            state = StringState::Escape;
            self.index += 1;
            self.column += 1;
          }
          // 其他字符
          _ => {
            self.index += 1;
            self.column += 1;
          }
        },
        // 转义字符
        StringState::Escape => {
          match c {
            // Unicode 字符
            'u' => {
              // 后面跟 4 位十六进制数字
              for i in 0..4 {
                if let Some(hex_c) = self.chars.get(self.index + i + 1) {
                  if is_hex(hex_c) {
                    self.index += 1;
                    self.column += 1;
                    continue;
                  }
                }

                return None;
              }

              state = StringState::QuoteOrChar;
            }
            // 其他转义字符
            '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {
              self.index += 1;
              self.column += 1;
              state = StringState::QuoteOrChar;
            }
            _ => return None,
          }
        }
        _ => return None,
      }
    }

    None
  }

  fn number(&mut self) -> Option<Token> {
    let mut state = NumberState::Start;
    let start_index = self.index;
    let mut parsed_index: usize = 0;

    while let Some(c) = self.chars.get(self.index) {
      match state {
        NumberState::Start => match c {
          '-' => {
            state = NumberState::Minus;
          }
          '0' => {
            state = NumberState::Zero;
          }
          '1'..='9' => {
            state = NumberState::Digit;
          }
          _ => break,
        },
        NumberState::Minus => match c {
          '0' => {
            state = NumberState::Zero;
            parsed_index = self.index;
          }
          '1'..='9' => {
            state = NumberState::Digit;
            parsed_index = self.index;
          }
          _ => break,
        },
        NumberState::Zero => match c {
          '.' => {
            state = NumberState::Point;
          }
          'e' | 'E' => {
            state = NumberState::Exp;
          }
          _ => break,
        },
        NumberState::Digit => match c {
          '0'..='9' => {
            parsed_index = self.index;
          }
          '.' => {
            state = NumberState::Point;
          }
          'e' | 'E' => {
            state = NumberState::Exp;
          }
          _ => break,
        },
        NumberState::Point => match c {
          '0'..='9' => {
            state = NumberState::Fraction;
            parsed_index = self.index;
          }
          _ => break,
        },
        NumberState::Fraction => match c {
          '0'..='9' => {
            state = NumberState::Fraction;
            parsed_index = self.index;
          }
          'e' | 'E' => {
            state = NumberState::Exp;
          }
          _ => break,
        },
        NumberState::Exp => match c {
          '-' => {
            state = NumberState::ExpSignOrDigit;
          }
          '0'..='9' => {
            state = NumberState::ExpSignOrDigit;
            parsed_index = self.index;
          }
          _ => break,
        },
        NumberState::ExpSignOrDigit => match c {
          '0'..='9' => {
            state = NumberState::Fraction;
            parsed_index = self.index
          }
          _ => break,
        },
      };

      self.index += 1;
      self.column += 1;
    }

    if parsed_index > 0 {
      let value = self
        .substring(start_index, parsed_index + 1)
        .parse::<f64>()
        .unwrap();
      let token = Token::Number(NumberToken {
        value,
        span: self.line_span(parsed_index + 1 - start_index),
      });

      return Some(token);
    }

    None
  }

  fn boolean(&mut self) -> Option<Token> {
    if self.substring(self.index, self.index + TRUE_LEN) == "true" {
      let token = Token::Boolean(BoolToken {
        value: true,
        span: self.line_span(TRUE_LEN),
      });
      self.index + TRUE_LEN;
      self.column + TRUE_LEN;

      return Some(token);
    }

    if self.substring(self.index, self.index + FALSE_LEN) == "false" {
      let token = Token::Boolean(BoolToken {
        value: false,
        span: self.line_span(FALSE_LEN),
      });
      self.index + FALSE_LEN;
      self.column + FALSE_LEN;

      return Some(token);
    }

    None
  }

  fn null(&mut self) -> Option<Token> {
    if self.substring(self.index, self.index + NULL_LEN) == "null" {
      let token = Token::Null(NullToken {
        span: self.line_span(NULL_LEN),
      });
      self.index + NULL_LEN;
      self.column + NULL_LEN;

      return Some(token);
    }

    None
  }
}

fn is_digit(c: &char) -> bool {
  *c >= '0' && *c <= '9'
}

fn is_digit_1_to_9(c: &char) -> bool {
  *c >= '1' && *c <= '9'
}

fn is_hex(c: &char) -> bool {
  is_digit(c) || *c >= 'a' && *c <= 'f' || *c >= 'A' && *c <= 'F'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut tokenizer = Tokenizer::new("{\"hello\": \"world\"}");
    let tokens = tokenizer.tokenize().unwrap();

    assert_eq!(tokens.len(), 5);

    assert_eq!(
      tokens,
      [
        Token::LeftBrace(LeftBraceToken {
          span: Span {
            start: Loc {
              line: 1,
              column: 1,
              offset: 0,
            },
            end: Loc {
              line: 1,
              column: 2,
              offset: 1,
            },
          },
        },),
        Token::String(StringToken {
          value: "\"hello\"".to_string(),
          span: Span {
            start: Loc {
              line: 1,
              column: 8,
              offset: 7,
            },
            end: Loc {
              line: 1,
              column: 15,
              offset: 14,
            },
          },
        },),
        Token::Colon(ColonToken {
          span: Span {
            start: Loc {
              line: 1,
              column: 9,
              offset: 8,
            },
            end: Loc {
              line: 1,
              column: 10,
              offset: 9,
            },
          },
        },),
        Token::String(StringToken {
          value: "\"world\"".to_string(),
          span: Span {
            start: Loc {
              line: 1,
              column: 17,
              offset: 16,
            },
            end: Loc {
              line: 1,
              column: 24,
              offset: 23,
            },
          },
        },),
        Token::RightBrace(RightBraceToken {
          span: Span {
            start: Loc {
              line: 1,
              column: 18,
              offset: 17,
            },
            end: Loc {
              line: 1,
              column: 19,
              offset: 18,
            },
          },
        },),
      ]
    );
  }
}
