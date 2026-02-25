
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
  Error,
  Warning,
  Info
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
  pub title: String,
  pub text: String,
  pub level: Level,
  pub location: Option<Location>,
  pub hint: Option<String>,
}


#[derive(Debug)]
pub struct Text {
  pub content : String,
  pub style   : TextFont,
  pub color   : u8
}

impl Text {
  pub fn new(content: &str) -> Text {
    Self {
      content : String::from(content),
      style   : TextFont::Regular,
      color   : 0x000000
    }
  }
  pub fn write(mut self, content: &str) -> Text {
    self.content = String::from(content);
    self
  }
  pub fn dye(mut self, color: u8) -> Text {
    self.color = color;
    self
  }
  pub fn style(mut self, font: TextFont) -> Text {
    self.style = font;
    self
  }
}

pub struct Palette {
  keywords  : u8,
  types     : u8,
  idents    : u8,
  strings   : u8,
  numbers   : u8,
  comments  : u8,
  operators : u8,
  function  : u8,
  colorful  : bool
}

/*
let rosepine = Palette {
  keywords  : Blue,
  types     : Cyan,
  idents    : White,
  strings   : Green,
  numbers   : Yellow,
  comments  : Gray,
  operators : Blue,
  function  : Pink
}
*/

impl Select {
  fn new(title: &str) -> Self {
    Select {
      title    : Text::new(title),
      subtitle : Text::new(""),
      options  : Vec::new()
    }
  }
  pub fn describe(mut self, text: &str) -> Self {
    self.subtitle = Text::new(text);
    self
  }
  pub fn option(mut self, question: &str) -> Self {
    self.options.push(
      Text::new(question)
    );
    self
  }
  pub fn display(self) {
    let mut index = 0;
    print!("{}", color::Fg(color::Red));
    println!("{}", self.title.content);
    print!("{}", color::Fg(color::Black));
    println!("{}", self.subtitle.content);
    print!("{}", color::Fg(color::Reset));

    for (i, text) in self.options.iter().enumerate() {
      println!("{}{}", { if i == index { " > " } else { "   " } }, text.content);
    }

    let stdin = stdin();
    let mut stdout = stdout();

    println!("Press keys (q to quit):");
    stdout.flush().unwrap();

    for c in stdin.keys() {
      match c.unwrap() {
        Key::Char('q') => break,
        Key::Char('\n') => {
          //println!("Pressed enter")
        }
        Key::Up => {
          if index == 0 {
            index = self.options.len() - 1;
          } else { index -= 1 }
        }
        Key::Down => {
          if index == self.options.len() - 1 {
            index = 0
          } else { index += 1 }
        }
        _ => {
          println!("Other key pressed")
        }
      }
      print!("{}{}", termion::cursor::Up(self.options.len() as u16 + 1), termion::clear::AfterCursor);
      for (i, text) in self.options.iter().enumerate() {
        println!("{}{}", { if i == index { " > " } else { "   " } }, text.content);
      }
    }
  }
}

fn main() {
  Select::new("How do you like your ice cream??")
  . describe("We have ice cream for everyone!!")
  . option("Vanilla?")
  . option("Chocolate?")
  . option("Lemon?")
  . display();

  println!("Welcome to frozen sweets!");
}
