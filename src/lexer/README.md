```
lexer/
├── tokens.rs
├── rules/
│   ├── string.rs
│   ├── character.rs
│   ├── comment.rs
│   ├── identifier.rs
│   ├── whitespace.rs
│   ├── delimeter.rs
│   ├── symbol.rs
│   └── number.rs
├── cursor/
│   ├── analysis.rs
│   └── stream.rs
└── tokens/
    ├── delimiter.rs
    ├── keywords.rs
    ├── literal.rs
    └── operator.rs
```

# Token Enums

In Mochick tokens are split in to four different files, `delimiters`, `keywords`, `literals` and `operators`. They are located in `tokens/` to keep things clean, this design might not be simple to implement since nested enums will add complexity, but we'll decide to that later! All of these tokens nested inside of the `TokenKind` enum at `tokens.rs` file, and the tokenizing process starts with `cursor/analysis.rs` file's `tokenize()` function.

```rs
enum WrapperKind {
  Paren,   // (...)
  Bracket, // [...]
  Squirly, // {...}
}

enum IdentKind {
  Lowercase,
  Localised,
  Discared
}
```

```rs
enum TokenKind {
  Ident(String, IdentKind),
  Comment(String),
  Whitespace(usize),
  Group { kind: WrapperKind, stream: TokenStream },
  EoL,
  EoF,
  Unknown(char)
}
```

- Identifiers has an `IdentKind` because of the way you name things does matter! For example an identifier starts with underscore will have `IdentKind::Discared`. This helps to understand how to process names.
- Groups are pretty rich tokens which helps to parser a lot with understanding scopes, they have their own kind and a `TokenStream` which is wrapped by them
