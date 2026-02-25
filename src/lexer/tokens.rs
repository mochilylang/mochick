pub mod delimiter;
pub mod keyword;
pub mod literal;
pub mod operator;

use crate::lexer::cursor::stream::TokenStream;

pub use delimiter :: Kind as Delimiter;
pub use keyword   :: Kind as Keyword;
pub use literal   :: Kind as Literal;
pub use operator  :: Kind as Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum WrapperKind {
  Paren,   // (...)
  Bracket, // [...]
  Squirly, // {...}
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentKind {
  Lowercase,
  Localised,
  Discared
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Op(operator::Kind),
  Kw(keyword::Kind),
  Lit(literal::Kind),
  Del(delimiter::Kind),

  Ident(String, IdentKind),
  Comment(String),
  Whitespace(usize),
  // Whitespaces hold numbers which refers their repetition count

  Group {
    kind   : WrapperKind,
    stream : TokenStream,
  },

  Newline,
  End,
  Unknown(char)
}
