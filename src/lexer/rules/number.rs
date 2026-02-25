use crate ::lexer ::{
  cursor :: {
    analysis::Lexlina,
  },
  tokens::{Literal, TokenKind, literal::Base},
};

pub fn tokenize(lexer: &mut Lexlina) -> TokenKind {
  let mut output: i64 = 0;
  let output_base = {
    if lexer.peek() == Some('0') {
      let result = match lexer.second() {
        Some('b' | 'B') => Base::Binary,
        Some('o' | 'O') => Base::Octal,
        Some('x' | 'X') => Base::Hexadecimal,
        Some('0'..='9')
        | Some(' ' | '\n') => Base::Decimal,
        _ => panic!("Invalid number base")
      };
      if result != Base::Decimal {
        lexer.advance();
        lexer.advance();
      }
      result
    }
    else {
      Base::Decimal
    }
  };
  let radix = output_base.clone() as i64;
  while let Some(chara) = lexer.peek() {
    match chara {
      '_' => {
        lexer.advance();
      }
      char => {
        let value = match char {
          '0' => 0, '1' => 1,
          '2' => 2, '3' => 3,
          '4' => 4, '5' => 5,
          '6' => 6, '7' => 7,
          '8' => 8, '9' => 9,
          'A' | 'a' => 10,
          'B' | 'b' => 11,
          'C' | 'c' => 12,
          'D' | 'd' => 13,
          'E' | 'e' => 14,
          'F' | 'f' => 15,
          ' ' | '\n' => break,
          char => panic!("Unexpected thingy in a number sequance {char}")
        };
        if value > radix {
          panic!("Invalid number!");
        }
        output = (output * radix) + value;
        lexer.advance();
      }
    }
  }
    /*if chara.is_ascii_digit() || chara == '.' || chara == '_' {
      lexer.advance();
      output = output * 10 + chara.parse().unwrap();
    } else {
      break;
    }*/
  /*if output(".") {
    TokenKind::Lit(Literal::Double {
      base: output_base,
      value: output
    })
  }*/
//else {
    TokenKind::Lit(Literal::Int {
      base  : output_base,
      value : output
    })
  //}
}
