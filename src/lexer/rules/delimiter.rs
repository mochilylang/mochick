use crate::Span;
use crate ::lexer ::{
  cursor ::{
    analysis::Lexlina,
    stream ::{ TokenStream, Token },
  },
  tokens::{TokenKind, Delimiter, WrapperKind},
};

pub fn wrap(mut stream: TokenStream) -> Result<TokenStream, String> {
  #[derive(Debug)]
  // This is a structure for groups which is still in process!!
  struct Trailing {
    pub expecting : WrapperKind,
    pub stream    : Vec<Token>,
    pub pin       : (usize, usize),
  }
  let mut content : Vec<Token> = Vec::new(); // Output
  let mut stack   : Vec<Trailing>  = Vec::new();

  // The stack will get updated on everytime when the loop finds a opening delimiter
  // It's named stack, because it's the samethingy for delimiters (Last In First Out)
  // Basicly if you write something like [ ( {...} ) ]
  // As you see the bracket opened first needs to be closed lastly!!

  // Let's start with a loop, we need to inspect every token passed
  while let Some(token) = stream.next() {
    use TokenKind::*;
    use Delimiter::*;
    // If the current token is kind of a delimiter...
    // We will continue with some information!
    let (delimiter, opening, closing) = match token.kind {
      Del(OpenParen) | Del(CloseParen) => (
        WrapperKind :: Paren,
        Del(OpenParen),
        Del(CloseParen),
      ),
      Del(OpenBracket) | Del(CloseBracket) => (
        WrapperKind :: Bracket,
        Del(OpenBracket),
        Del(CloseBracket),
      ),
      Del(OpenSquirly) | Del(CloseSquirly) => (
        WrapperKind :: Squirly,
        Del(OpenSquirly),
        Del(CloseSquirly),
      ),
      _ => {
        // But what if the token isn't kind of a delimiter??
        // If the stack is not empty, current token needs to be pushed in to
        // latest group's stream from the stack!! But if the stack is empty,
        // that means current token is not covered by some group so it will be pushed in to 'content'
        if let Some(group) = stack.last_mut() {
          group.stream.push(token);
        } else {
          content.push(token);
        }
        continue;
      },
    };
    if token.kind == opening {
      stack.push(Trailing {
        expecting : delimiter,
        stream    : Vec::new(),
        pin       : token.loc.start
      });
    }
    else if token.kind == closing {
      if let Some(closed) = stack.pop() {
        // Don't forget that we used .pop on the 'stack' before the process
        // It means if you try to get the latest element in the stack
        // you'll get the 2nd element from the last.
        if closed.expecting != delimiter {
          let mut quit = false;
          for unclosed in stack.iter().rev() {
            if unclosed.expecting == delimiter {
              println!("Forgot to close one delimiter: {:?}", closed.expecting);
              quit = true;
              break;
            }
          }
          if quit {
            stack.pop();
            continue;
          }
          println!(
            "Expected {:?} but found {:?} instead",
            closed.expecting, delimiter
          );
          continue;
        }
        let group = Token {
          kind: Group {
            stream : TokenStream::new(closed.stream),
            kind   : delimiter
          },
          loc    : Span {
            start : closed.pin,
            end   : token.loc.end
          }
        };
        // If a group is alone in the stack it can pushed in to content directly!!
        if stack.is_empty() {
          content.push(group);
        } else {
          // But if it's not, it will be pushed in to previous group's content
          println!("{:?}", stack.len());
          let previous = stack.len() - 1;
          stack
          . get_mut(previous)
          . unwrap().stream
          . push(group);
        }
        continue;
      } else {
        // This block works when the delimiter closed without opening it -> ...]
        panic!(
          "You tried to close a {:?} but it was never opened! Missing {:?}",
          delimiter, opening
        )
      }
    } else {
      print!("Testing somethingy!")
    }
  }
  // If stack is still full after we inspected all tokens,
  // it means there is some delimiters which is not closed!
  for i in stack {
    println!("Expected: {:?} at {:?}", i.expecting, i.pin);
  }

  Ok(TokenStream {
    stream: content,
    cursor: 0,
    position: (1, 0)
  })
}
