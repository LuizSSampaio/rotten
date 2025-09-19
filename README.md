# Rotten — a tiny teaching language in Rust

Rotten is a compact, educational implementation of a programming language inspired by the Craft interpreter book. It’s written in Rust and includes a small lexer, a hand-written parser, and a simple interpreter with lexical scoping, classes, inheritance, and a tiny standard environment.

## Features

- Variables, functions, and control flow (if, while, for)
- Classes with inheritance, this/super, and simple method binding
- Lexical scoping with a lightweight environment model
- A tiny standard library including a built-in print function
- A REPL for quick experiments
- Simple error reporting and recoverable parse/runtime errors

## Quick Start

### Prerequisites

- Rust toolchain (stable, 1.XX+)
- Cargo

### Build

- `cargo build`

### Run the REPL

- `cargo run`

### Run a script

- Create a `.rot` file and run:
- `cargo run -- path/to/your_script.rot`
- The interpreter prints results to stdout; errors are shown in the terminal.

## Hello World example

```rot
print("Hello Rotten!");
```

The built-in print function outputs the argument to stdout. Any expression that evaluates to a value can be printed, e.g. a number, string, or boolean.

## Small language snapshot

Here are a few sample snippets to illustrate typical usage.

### Variables

```rot
var x = 42;
print(x);
```

### Functions

```rot
fun add(a, b) {
  return a + b;
}

var result = add(3, 5);
print(result);
```

### Objects and Classes

```rot
class Person {
  greet() {
    print("Hello, I am " + this.name);
  }
}

var p = Person();
p.name = "Ada";
p.greet();
```

### Inheritance

```rot
class Animal {
  speak() {
    print("generic sound");
  }
}

class Dog < Animal {
  speak() {
    super.speak();
    print(" (woof)");
  }
}

var d = Dog();
d.speak();
```

### Control flow

```rot
var i = 0;
while (i < 3) {
  print(i);
  i = i + 1;
}
```

## Language design notes

- Rotten is a direct, beginner-friendly interpreter with a focus on clarity and small surface area.
- It follows a typical Craft-style structure: a scanner that produces tokens, a parser that builds ASTs, and an interpreter that evaluates code via a visitor pattern over expressions and statements.
- The runtime supports basic object-oriented features, including method binding and simple inheritance with `this` and `super`.

## Project structure

- `src/lexer/` — lexical analysis (scanner, reader, emitter, keywords, errors)
- `src/parser/` — parser and AST definitions (expressions, statements, error handling)
- `src/interpreter/` — runtime and environment management, built-ins, and the visitor implementations
- `src/memory/` — lexical environments and memory handlers
- `src/token/` — token kinds, positions, and value representations
- `src/main.rs` — entry point (REPL or script runner)

## How the interpreter works (high-level)

- Tokens are produced by the scanner from source text.
- The parser builds an AST of statements and expressions.
- The interpreter walks the AST using the visitor pattern, maintaining a runtime environment with lexical scoping.
- Classes are represented as data structures with a map of methods; objects are instances bound to their class and environment.
- A built-in `print` function demonstrates interaction with the host environment and I/O.

## Testing & formatting

- Build and run tests: `cargo test`
- Format code: `cargo fmt` (as with Rust projects)
- Lint: `cargo clippy --all-targets -- -D warnings`

## Acknowledgements

This repo is a learning project inspired by the Craft interpreter book. It’s designed to be approachable and extensible for experimentation and education.
