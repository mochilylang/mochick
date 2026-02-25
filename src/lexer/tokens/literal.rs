#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  Bool { value: bool },
  Char { value: char },

  Int    { base: Base, value: i64 },
  Double { base: Base, value: f64 },

  Str { value: String, raw: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Base {
  Binary      =  2, // Literal starts with "0b"
  Octal       =  8, // Literal starts with "0o"
  Decimal     = 10, // Literal doesn't contain a prefix.
  Hexadecimal = 16  // Literal starts with "0x"
}
