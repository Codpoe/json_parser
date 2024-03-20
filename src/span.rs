#[derive(Debug, PartialEq, Clone, Default)]
pub struct Loc {
  pub line: usize,
  pub column: usize,
  pub offset: usize,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Span {
  pub start: Loc,
  pub end: Loc,
}
