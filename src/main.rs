use clap::{Arg, ArgAction, Command};
use mochick::lexer::cursor::analysis::Lexlina as lexer;
use mochick::lexer::cursor::stream::{Token, TokenStream};
use mochick::lexer::tokens::TokenKind;
use mochick::parser::parser::Parser;

fn main() {
  let matches = Command::new("mochick")
  . bin_name("mochick")
  . arg_required_else_help(true)
  . arg(Arg::new("file")
        . help("File path")
        . action(ArgAction::Set)
        . required(true))
  . get_matches();

  match matches.get_one::<String>("file") {
    Some(path) => {
      let contents = std::fs::read_to_string(path)
      . expect("Failed to read the file");

      let mut lexer = lexer::new(contents);
      let mut spans: TokenStream = TokenStream {
        position: (0, 0),
        stream: Vec::new(),
        cursor: 0,
      };
      loop {
        if let Ok(x) = lexer.tokenize() {
          match x.kind {
            TokenKind::End => break,
            _ => {
              println!("{:?}", x.kind);
              spans.stream.push(x);
            }
          }
        }
        else {
          println!("Lexer error");
        }
      }

      let mut parser = Parser::new(spans);
      match parser.parse() {
        ast => {
          std::fs::write("out.ron", format!("{:#?}",ast))
          . expect("Failed to read the file");
        }
      }
    },
    _ => unreachable!(),
  }
}
