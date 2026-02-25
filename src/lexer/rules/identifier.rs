use crate ::lexer ::{
  cursor::analysis::Lexlina,
  tokens::{IdentKind, Keyword, TokenKind}
};

fn is_fitting(next: char) -> bool {
 next.is_ascii_alphanumeric() || next == '_' || next == '-'
}

pub fn tokenize(lexer: &mut Lexlina) -> TokenKind {
  let mut ident = String::new();

  while let Some(next) = lexer.peek() {
    if is_fitting(next) {
      ident.push(next);
      lexer.advance();
    } else {
      break;
    }
  };

  use Keyword::*;
  let keyword = match ident.as_str() {
    "import" => Import,
    "extern" => Extern,
    "priv"   => Priv,
    "fn"     => Fn,
    "alt"    => Alt,
    "return" => Return,
    "echo"   => Echo,
    "enum"   => Enum,
    "schema" => Schema,
    "type"   => Type,
    "module" => Module,
    "if"     => If,
    "else"   => Else,
    "case"   => Case,
    "while"  => While,
    "until"  => Until,
    "let"    => Let,
    "const"  => Const,
    "or"     => Or,
    "and"    => And,
    _ => return TokenKind::Ident(ident, IdentKind::Lowercase)
  };
  TokenKind::Kw(keyword)
}
