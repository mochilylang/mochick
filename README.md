<div align="center">
  <picture>
    <img alt="Mochick The Mochily Compiler"
      src="repository/mochi.png"
      width="128px">
  </picture>
</div>

# Work In Progress

To test the language, first make sure [Rust](https://rust-lang.org/learn/get-started) is installed, then you could run
```
cargo run -- test/main.ly
```
This command will generate a file named `out.ron` in the project root to represent the AST output of your program!

You can see the file structure tree below, please also note that there is a `README.md` file for each directory for better explainings!

src/ \
├── ast/ **AST** \
│   ├── nodes/   *Symbol Table* \
│   ├── printer/ *Pretty Printer* \
│   └── walker/  *AST Walker* \
├── codegen/ **Targets** \
│   └── javascript/ \
├── lexer/ **Lexical Analysis** \
│   ├── tokens.rs \
│   ├── rules/ *Rules for each tokens* \
│   │   ├── string.rs \
│   │   ├── character.rs \
│   │   ├── comment.rs \
│   │   ├── identifier.rs \
│   │   ├── whitespace.rs \
│   │   ├── delimeter.rs \
│   │   ├── symbol.rs \
│   │   └── number.rs \
│   ├── cursor/ *Tokenizer* \
│   │   ├── main.rs \
│   │   └── stream.rs \
│   └── tokens/ *Tokens definitions* \
│       ├── literals.rs \
│       ├── operators.rs \
│       ├── delimiters.rs \
│       └── keywords.rs \
├── parser/ **Parser** \
│   ├── grammer/ \
│   └── walker/ \
├── reporter/ **Error Reporter** \
│   ├── diagnostics/ *Error Types* \
│   │   ├── logic.rs \
│   │   ├── semantic.rs \
│   │   └── syntax.rs \
│   └── formatter/ *Printing Errors* \
└── validator/ **Semantic Analysis** \
    ├── resolve/ *Name Resolver* \
    └── typing/  *Type Checker* \
