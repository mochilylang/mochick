use crate ::lexer ::
  tokens ::TokenKind;
use crate::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  pub kind : TokenKind,
  pub loc  : Span
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenStream {
  pub stream: Vec<Token>,
  pub cursor: usize,
  pub position: (usize, usize)
}

impl TokenStream {
  /*
    advance - Move the cursor without returning anything.
    peek    - Look ahead at future token without consuming them.
    next    - Return current token and advance parser.
    get     - Get nth token without consuming.

    push - Push a tokentree in stream

    from - Take a string and tokenize it
  */

  pub fn new(input: Vec<Token>) -> Self {
    TokenStream {
      stream: input,
      cursor: 0,
      position: (1, 0)
    }
  }

  pub fn advance(&mut self) {
    self.cursor += 1;
    self.position.0 += 1;
  }

  pub fn peek(&self) -> Option<Token> {
    self.stream.get(self.cursor).cloned()
  }

  pub fn next(&mut self) -> Option<Token> {
    let out = self.peek();
    self.advance();
    out
  }
}
