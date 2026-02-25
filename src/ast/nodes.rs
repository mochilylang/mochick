use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
  String,
  Void,
  Int,
  Defined(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Int {
    loc   : Span,
    value : i64,
  },

  String {
    loc   : Span,
    type_ : Type,
    value : String,
  },

  Block {
    loc  : Span,
    body : Vec<Expression>,
  },

  Fn {
    loc    : Span,
    type_  : Type,
    params : Vec<Type>,
    body   : Vec<Expression>,
    retur  : Option<Type>,
  },

  Assignment {
    loc    : Span,
    type_  : Type,
    value  : Box<Expression>,
    name   : String,
  }
}
