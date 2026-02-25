use crate::lexer::tokens::{Keyword, TokenKind};
use crate::ast::nodes::{Expression, Type};
use crate::lexer::cursor::stream::{ self, Token, TokenStream };
use crate::Span;
use crate::parser::grammer::assignment;

#[derive(Debug)]
pub struct Parser {
  pub stream: TokenStream,
}

impl Parser {
  pub fn new(stream: TokenStream) -> Self {
    Self { stream }
  }
  pub fn next(&mut self) -> Option<Token> {
    if self.stream.peek().unwrap().kind == TokenKind::Whitespace(1) {
      self.stream.advance();
    };
    self.stream.next()
  }
  pub fn peek(&mut self) -> Option<Token> {
    if self.stream.peek().unwrap().kind == TokenKind::Whitespace(1) {
      self.stream.advance();
    };
    self.stream.peek()
  }
  pub fn parse(&mut self) -> Expression {
    if let Some(next) = self.next() {
      match next.kind {
        TokenKind::Kw(Keyword::Let) => assignment::parse(self),
        _ =>
        Expression::Block { loc: Span { start: (1, 2), end: (1, 2) }, body: Vec::new() }
      }
    }
    else {
      Expression::Block { loc: Span { start: (1, 2), end: (1, 2) }, body: Vec::new() }
    }
  }
}
