# XL Compiler (Rust)

This is a proof-of-concept compiler implementation for the experimental **XL** programming language, built in Rust.

## Prerequisites

* Rust toolchain (edition 2021) — install via [rustup](https://rustup.rs)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run -- path/to/source.xl -o output.txt
```

For now the compiler lexes, parses, and pretty-prints the generated AST. Future work will add semantic analysis and LLVM-based code generation to produce native executables.