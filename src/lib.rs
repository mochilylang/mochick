pub mod lexer {
  pub mod rules {
    pub mod character;
    pub mod comment;
    pub mod delimiter;
    pub mod identifier;
    pub mod number;
    pub mod string;
    pub mod symbol;
    pub mod whitespace;
  }
  pub mod cursor {
    pub mod analysis;
    pub mod stream;
  }
  pub mod tokens;
}

pub mod ast {
  pub mod nodes;
}

pub mod parser {
  pub mod parser;
  pub mod grammer {
    pub mod assignment;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
  pub start : (usize, usize), // (Line, Column)
  pub end   : (usize, usize)
}
