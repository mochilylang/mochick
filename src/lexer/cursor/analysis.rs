use crate::Span;
use crate ::lexer ::{
  cursor ::
    stream ::Token, rules::{
    character, identifier, number, string
  }, tokens::{Delimiter, Operator, TokenKind}
};

#[derive(Debug)]
pub struct Lexlina {
  pub stream   : Vec<char>,
  pub cursor   : usize,
  pub position : (usize, usize),
}

#[derive(Debug)]
pub struct LexerError {
}

impl Lexlina {
  pub fn new(input: String) -> Self {
    Self { stream: input.chars().collect(), position: (1, 0), cursor: 0 }
  }
  pub fn advance(&mut self) {
    self.cursor += 1;
    self.position.1 += 1;
  }

  pub fn peek(&self) -> Option<char> {
    self.stream.get(self.cursor).copied()
  }

  pub fn prev(&mut self) -> Option<char> {
    self.stream.get(self.cursor - 1).copied()
  }
  pub fn next(&mut self) -> Option<char> {
    let char = self.peek();
    self.advance();
    char
  }
  pub fn second(&mut self) -> Option<char> {
    self.stream.get(self.cursor + 1).copied()
  }

  fn stack(&mut self, chara: char) -> usize {
    let mut stack: usize = 0;
    while let Some(next) = self.peek() {
      if next == chara {
        stack += 1;
        self.advance();
        continue;
      }
      break;
    }
    stack
  }

  fn combine(&mut self, patterns: &[(&str, TokenKind)]) -> TokenKind {
    for (pat, val) in patterns.iter().rev() {
      let mut chars = pat.chars();
      let mut matched = true;
      chars.next();

      for expected in chars {
        match self.peek() {
          Some(c) if c == expected => {
            self.advance();
          },
          _ => {
            matched = false;
            break;
          },
        }
      }

      if matched {
        return val.clone();
      }
    }
    TokenKind::Op(Operator::At)
  }

  pub fn tokenize(&mut self) -> Result<Token, String> {
    let char = match self.peek() {
      Some(c) => c,
      None => {
        let pose = (self.position.0 - 1, self.position.1);
        return Ok(Token {
          kind : TokenKind::End,
          loc  : Span {
            start : pose,
            end   : pose
          }
        });
      },
    };
    use TokenKind::Op;
    use TokenKind::Del;
    use Operator::*;
    use Delimiter::*;
    let pin = self.position;
    let token = match char {
      '@' => Op(At),
      '#' => Op(Pound),
      '$' => Op(Dollar),
      '~' => Op(Tilde),
      '&' => Op(Ampersand),
      '|' => self.combine(&[
        ("|",  Op(Pipe)),
        ("|>", Op(Pipeline))
      ]),
      '!' => self.combine(&[
        ("!",  Op(Bang)),
        ("!=", Op(NotEqual))
      ]),
      '?' => Op(Question),
      ',' => Op(Coma),
      '.' => self.combine(&[
        (".",   Op(Dot)),
        ("..",  Op(DotDot)),
        ("...", Op(Ellipsis))
      ]),
      '"'  => string::tokenize(self),
      '\'' => character::tokenize(self),
      //'`'  => Op(Backtick),
      ':'  => Op(Colon),
      '(' => Del(OpenParen),
      ')' => Del(CloseParen),
      '[' => Del(OpenBracket),
      ']' => Del(CloseBracket),
      '{' => Del(OpenSquirly),
      '}' => Del(CloseSquirly),
      '+' => Op(Plus),
      '-' => self.combine(&[
        ("-",  Op(Dash)),
        ("->", Op(RightArrow))
      ]),
      '*' => Op(Star),
      '/' => Op(Slash),
      '%' => Op(Percent),
      '^' => Op(Caret),
      '<' => self.combine(&[
        ("<",   Op(Greater)),
        ("<=>", Op(Spaceship))
      ]),
      '>' => self.combine(&[
        (">",   Op(Greater)),
        (">=",  Op(GreaterEqual)),
      ]),
      '=' => self.combine(&[
        ("=",  Op(Equal)),
        ("=<", Op(LessEqual)),
        ("=>", Op(FatArrow))
      ]),
      '0'..='9' => number::tokenize(self),
      char if char.is_ascii_alphabetic() || char == '_' => identifier::tokenize(self),
      ' ' => TokenKind::Whitespace(self.stack(' ')),
      '\n' => {
        self.position.1 -= 1;
        let start = self.position;

        let stack = self.stack('\n');
        self.position = (1, self.position.0 + stack);

        let end = self.position;
        self.position.1 -= 1;

        return Ok(Token {
          kind: TokenKind::Newline,
          loc: Span {
            start,
            end
          }
        });
      },
      _ => TokenKind::Unknown(char),
    };
    match token {
      Op(_) => self.advance(),
      Del(_) => self.advance(),
      _  => {}
    }
    Ok(Token {
      kind: token,
      loc: Span {
        start: pin,
        end: self.position
      }
    })
  }
}
