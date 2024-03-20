# json_parser

A JSON parser written in Rust. Just for fun.

## AST

```json
{
  "hello": "world"
}
```

The corresponding AST is:

```rust
Object(
    ObjectAst {
        value: [
            PropertyAst {
                key: IdentifierAst {
                    value: StringAst {
                        value: "hello",
                        span: Span {
                            start: Loc {
                                line: 2,
                                column: 3,
                                offset: 4,
                            },
                            end: Loc {
                                line: 2,
                                column: 10,
                                offset: 11,
                            },
                        },
                    },
                    span: Span {
                        start: Loc {
                            line: 2,
                            column: 3,
                            offset: 4,
                        },
                        end: Loc {
                            line: 2,
                            column: 10,
                            offset: 11,
                        },
                    },
                },
                value: String(
                    StringAst {
                        value: "world",
                        span: Span {
                            start: Loc {
                                line: 2,
                                column: 12,
                                offset: 13,
                            },
                            end: Loc {
                                line: 2,
                                column: 19,
                                offset: 20,
                            },
                        },
                    },
                ),
                span: Span {
                    start: Loc {
                        line: 2,
                        column: 3,
                        offset: 4,
                    },
                    end: Loc {
                        line: 2,
                        column: 19,
                        offset: 20,
                    },
                },
            },
        ],
        span: Span {
            start: Loc {
                line: 1,
                column: 1,
                offset: 0,
            },
            end: Loc {
                line: 3,
                column: 2,
                offset: 22,
            },
        },
    },
)
```

## Visit

You can visit or modify the nodes of a JSON AST by implementing the `visit` trait.

```rust
struct Visitor {
  pub property_pos: (usize, usize), // (start, end)
  pub merged_string: String, // xxx_yyy
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

fn test_visit() {
  let mut json = "{\"hello\":\"world\"}".parse::<Json>().unwrap();

  let mut visitor = Visitor {
    property_pos: (0, 0),
    merged_string: String::new(),
  };

  visitor.visit_json(&mut json);
  assert_eq!(visitor.property_pos, (1, 16));
  assert_eq!(visitor.merged_string, "hello_world");
}
```
