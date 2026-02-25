use crate ::lexer ::{
  cursor :: {
    analysis::Lexlina,
  },
  // rules::delimiter,
  tokens::{ TokenKind, Literal },
};

pub fn lexunico(lexer: &mut Lexlina) -> char {
  'a'
}

pub fn tokenize(lexer: &mut Lexlina) -> TokenKind {
  let output  : char;
  let mut raw = String::new();

  // Check if character literal starts with single quotes as expected.
  if let Some(next) = lexer.peek() {
    match next {
      '\'' => lexer.advance(),
      otherwise => panic!("Expected `'` found {:?}", otherwise)
    }
  }

  // If literal contains a backslash handle character escapes
  if lexer.peek() == Some('\\') {
    lexer.advance();
    output = match lexer.peek() {
      Some('n')  => '\n',
      Some('r')  => '\r',
      Some('t')  => '\t',
      Some('u')  => 'u',
      Some('0')  => '\0',
      Some('\'') => '\'',
      Some('\\') => '\\',
      _ => panic!("Unknown character escape")
      // Hint: Backslash is an escape character, if you meant to use it as a char write '\\' instead.
    };
    lexer.advance();
  } else {
    match lexer.next() {
      Some('\'') => panic!("Literal should not be empty!"),
      None | Some('\n') => panic!("BBBBBBB"),
      Some(char) => output = char
    }
  }
  raw.push(output);
  // Even output is initialized, there should be a loop to find termination delimiter
  while let Some(char) = lexer.next() {
    match char {
      '\'' => break,
      '\n' => panic!("Missing trailing `'` symbol to terminate the character literal (3)"),
      _ => raw.push(char),
    }
  }
  if raw.len() > 1 {
    if raw.chars().nth(1) == Some(' ') && raw.len() == 2 {
      panic!("Sweety, you added an extra space!");
    }
    panic!("Character literal may only contain one codepoint");
  }
  use TokenKind::Lit;
  return Lit(Literal::Char {
    value: output
  })
}
