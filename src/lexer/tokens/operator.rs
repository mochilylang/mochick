#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  At,     // @
  Pound,  // #
  Dollar, // $
  Tilde,  // ~

  Ampersand, // &
  Pipe,      // |
  Slash,     // /
  Pipeline,  // |>

  Bang,      // !
  Question,  // ?
  DotDot,    // ..
  Ellipsis,  // ...

  Plus,    // +
  Dash,    // -
  Star,    // *
  Percent, // %
  Caret,   // ^

  Less,    // <
  Greater, // >

  LessEqual,    // <=
  GreaterEqual, // >=

  Equal,    // =
  NotEqual, // !=

  Colon, // :
  Dot,   // .
  Coma      ,

  Spaceship,  // <=>
  FatArrow,   // =>
  LeftArrow,  // <-
  RightArrow, // ->
}
