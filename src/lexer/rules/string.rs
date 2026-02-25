use crate ::lexer ::{
  cursor :: {
    analysis::Lexlina,
  },
  tokens::{Literal, TokenKind},
};

pub fn tokenize(lexer: &mut Lexlina) -> TokenKind {
  let mut output = String::new();
  let is_raw = lexer.prev() == Some('r');
  if let Some(next) = lexer.peek() {
    match next {
      '"' => lexer.advance(),
      otherwise => panic!("Expected `'` found {:?}", otherwise)
    }
  }
  while let Some(chara) = lexer.next() {
    if chara == '\\' && !is_raw {
      match lexer.next() {
        Some('n')  => output.push('\n'),
        Some('r')  => output.push('\r'),
        Some('t')  => output.push('\t'),
        Some('u')  => output.push('u'),
        Some('0')  => output.push('\0'),
        Some('"') => output.push('\"'),
        Some('\\') => output.push('\\'),
        Some(_) => panic!("Unknown character escape"),
        None    => panic!("Missing trailing `'` symbol to terminate the character literal")
      };
    }
    else if chara == '"' {
      break;
    }
    else {
      output.push(chara);
    }
  }
  use TokenKind::Lit;
  return Lit(Literal::Str {
    value : output,
    raw   : is_raw
  })
}
