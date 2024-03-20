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

impl Token {
  pub fn get_span(&self) -> &Span {
    match self {
      Token::LeftBrace(token) => &token.span,
      Token::RightBrace(token) => &token.span,
      Token::LeftBracket(token) => &token.span,
      Token::RightBracket(token) => &token.span,
      Token::Colon(token) => &token.span,
      Token::Comma(token) => &token.span,
      Token::String(token) => &token.span,
      Token::Number(token) => &token.span,
      Token::Boolean(token) => &token.span,
      Token::Null(token) => &token.span,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct LeftBraceToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct RightBraceToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct LeftBracketToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct RightBracketToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ColBracketToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ColonToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct CommaToken {
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct StringToken {
  pub value: String,
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NumberToken {
  pub value: f64,
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct BoolToken {
  pub value: bool,
  pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NullToken {
  pub span: Span,
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

pub struct Tokenizer {
  chars: Vec<char>,
  len: usize,
  index: usize,
  line: usize,
  column: usize,
}

impl Tokenizer {
  pub fn new(input: &str) -> Self {
    let chars = input.chars().collect::<Vec<char>>();
    let len = chars.len();

    Self {
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
          "Unexpected char {:#?} at {:#?}:{:#?}",
          self.chars.get(self.index).unwrap(),
          self.line,
          self.column
        ));
      }
    }

    Ok(tokens)
  }

  fn line_span(&self, start_loc: Option<Loc>, end_index: usize) -> Span {
    let start_loc = start_loc.unwrap_or(Loc {
      line: self.line,
      column: self.column,
      offset: self.index,
    });

    let span_len = end_index - start_loc.offset;

    Span {
      end: Loc {
        line: start_loc.line,
        column: start_loc.column + span_len,
        offset: start_loc.offset + span_len,
      },
      start: start_loc,
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
          span: self.line_span(None, self.index + 1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      '}' => {
        let token = Token::RightBrace(RightBraceToken {
          span: self.line_span(None, self.index + 1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      '[' => {
        let token = Token::LeftBracket(LeftBracketToken {
          span: self.line_span(None, self.index + 1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      ']' => {
        let token = Token::RightBracket(RightBracketToken {
          span: self.line_span(None, self.index + 1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      ':' => {
        let token = Token::Colon(ColonToken {
          span: self.line_span(None, self.index + 1),
        });
        self.index += 1;
        self.column += 1;

        Some(token)
      }
      ',' => {
        let token = Token::Comma(CommaToken {
          span: self.line_span(None, self.index + 1),
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
    let start_loc = Loc {
      line: self.line,
      column: self.column,
      offset: self.index,
    };

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
              value: self.substring(start_loc.offset, self.index + 1),
              span: self.line_span(Some(start_loc), self.index + 1),
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
      }
    }

    None
  }

  fn number(&mut self) -> Option<Token> {
    let mut state = NumberState::Start;
    let mut parsed_index: usize = 0;
    let start_loc = Loc {
      line: self.line,
      column: self.column,
      offset: self.index,
    };

    while let Some(c) = self.chars.get(self.index) {
      match state {
        NumberState::Start => match c {
          '-' => {
            state = NumberState::Minus;
          }
          '0' => {
            state = NumberState::Zero;
            parsed_index = self.index + 1;
          }
          '1'..='9' => {
            state = NumberState::Digit;
            parsed_index = self.index + 1;
          }
          _ => break,
        },
        NumberState::Minus => match c {
          '0' => {
            state = NumberState::Zero;
            parsed_index = self.index + 1;
          }
          '1'..='9' => {
            state = NumberState::Digit;
            parsed_index = self.index + 1;
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
            parsed_index = self.index + 1;
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
            parsed_index = self.index + 1;
          }
          _ => break,
        },
        NumberState::Fraction => match c {
          '0'..='9' => {
            state = NumberState::Fraction;
            parsed_index = self.index + 1;
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
            parsed_index = self.index + 1;
          }
          _ => break,
        },
        NumberState::ExpSignOrDigit => match c {
          '0'..='9' => {
            state = NumberState::Fraction;
            parsed_index = self.index + 1;
          }
          _ => break,
        },
      };

      self.index += 1;
      self.column += 1;
    }

    if parsed_index > 0 {
      let value = self
        .substring(start_loc.offset, parsed_index)
        .parse::<f64>()
        .unwrap();
      let token = Token::Number(NumberToken {
        value,
        span: self.line_span(Some(start_loc), parsed_index),
      });

      return Some(token);
    }

    None
  }

  fn boolean(&mut self) -> Option<Token> {
    if self.substring(self.index, self.index + TRUE_LEN) == "true" {
      let token = Token::Boolean(BoolToken {
        value: true,
        span: self.line_span(None, self.index + TRUE_LEN),
      });
      self.index += TRUE_LEN;
      self.column += TRUE_LEN;

      return Some(token);
    }

    if self.substring(self.index, self.index + FALSE_LEN) == "false" {
      let token = Token::Boolean(BoolToken {
        value: false,
        span: self.line_span(None, self.index + FALSE_LEN),
      });
      self.index += FALSE_LEN;
      self.column += FALSE_LEN;

      return Some(token);
    }

    None
  }

  fn null(&mut self) -> Option<Token> {
    if self.substring(self.index, self.index + NULL_LEN) == "null" {
      let token = Token::Null(NullToken {
        span: self.line_span(None, self.index + NULL_LEN),
      });
      self.index += NULL_LEN;
      self.column += NULL_LEN;

      return Some(token);
    }

    None
  }
}

fn is_hex(c: &char) -> bool {
  *c >= '0' && *c <= '9' || *c >= 'a' && *c <= 'f' || *c >= 'A' && *c <= 'F'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenizer() {
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
              offset: 0
            },
            end: Loc {
              line: 1,
              column: 2,
              offset: 1
            }
          }
        }),
        Token::String(StringToken {
          value: "\"hello\"".to_string(),
          span: Span {
            start: Loc {
              line: 1,
              column: 2,
              offset: 1
            },
            end: Loc {
              line: 1,
              column: 9,
              offset: 8
            }
          }
        }),
        Token::Colon(ColonToken {
          span: Span {
            start: Loc {
              line: 1,
              column: 9,
              offset: 8
            },
            end: Loc {
              line: 1,
              column: 10,
              offset: 9
            }
          }
        }),
        Token::String(StringToken {
          value: "\"world\"".to_string(),
          span: Span {
            start: Loc {
              line: 1,
              column: 11,
              offset: 10
            },
            end: Loc {
              line: 1,
              column: 18,
              offset: 17
            }
          }
        }),
        Token::RightBrace(RightBraceToken {
          span: Span {
            start: Loc {
              line: 1,
              column: 18,
              offset: 17
            },
            end: Loc {
              line: 1,
              column: 19,
              offset: 18
            }
          }
        })
      ]
    );
  }
}
