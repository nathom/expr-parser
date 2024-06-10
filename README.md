# Arithmetic Expression Parser

A dependency-free lexer, parser, and evaluator for integer arithmetic
expressions.

It uses predictive, top-down, recursive descent parsing by implementing the
LL(1) expression grammar given in `Expr.abnf`. Thus, with only a single token of
lookahead, we are able to evaluate an arbitrary expression with correct rules of
precedence.

## Usage

Running the program will show the parse tree and result for the input.

```bash
cargo run
```

Type in the input, and hit return.

## Run tests

To run all tests, run

```bash
cargo test
```
