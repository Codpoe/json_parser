# json_parser

A JSON parser written in Rust. Just for fun.

For example:

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
