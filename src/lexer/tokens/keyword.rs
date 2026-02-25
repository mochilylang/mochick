#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  Import,
  Extern,
  Priv,

  Fn,
  Alt,
  Return,
  Yield,
  Echo,

  Enum,
  Schema,
  Type,
  Module,

  If,
  Else,
  When,
  Case,

  While,
  Until,
  Break,
  Skip,

  Let,
  Const,

  Or,
  And,
}
