use clap::builder::Str;

use crate::{ast::nodes::Expression, parser::parser::Parser};
use crate::Span;
use crate::lexer::tokens::{IdentKind, Keyword, Literal, Operator, TokenKind};
use crate::ast::nodes::{Type};
use crate::lexer::cursor::stream::{ self, Token, TokenStream };
use crate::parser::grammer::assignment;


pub fn parse(stream: &mut Parser) -> Expression {
  use TokenKind::*;
  let start = stream.peek().unwrap().loc.start;
  let name = if let Some(next) = stream.next() {
    match next.kind {
      Ident(some, IdentKind::Lowercase) => some,
      _ => String::new()
    }
  } else {
    panic!("Let what?")
  };

  match stream.peek().unwrap().kind {
    Op(Operator::Colon) => stream.stream.advance(),
    other => panic!("Expected colon (:), found {:?}", other)
  }

  let value = match stream.peek().unwrap().kind {
    Lit(Literal::Int { base, value }) => value,
    other => panic!("Expected a value, found {:?}", other)
  };

  let end = stream.next().unwrap().loc;
  Expression::Assignment {
    loc: Span { start, end: end.end, },
    type_: Type::Int,
    value: Box::new(Expression::Int { loc: end, value }),
    name
  }
}
