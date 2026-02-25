#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  // Quotes,     // "
  // Apostrophe, // '
  // Backtick,   // `

  OpenParen,  // (
  CloseParen, // )

  OpenBracket,  // [
  CloseBracket, // ]

  OpenSquirly,  // {
  CloseSquirly, // }
}
