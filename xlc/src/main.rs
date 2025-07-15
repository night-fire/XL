use clap::Parser;
use std::path::PathBuf;

mod lexer;
mod token;
mod ast;
mod parser;
mod semantic;
mod codegen;
mod error;
mod ast_utils;
mod parse_primitives;
mod passes;
mod visitor;

use anyhow::Result;

/// XL Compiler command-line arguments
#[derive(Parser, Debug)]
#[command(
    name = "xlc",
    version = "0.1.0",
    about = "XL compiler implemented in Rust"
)]
struct Args {
    /// Source file to compile
    #[arg(value_name = "SOURCE")] 
    input: PathBuf,

    /// Output executable or IR file path
    #[arg(short, long, value_name = "OUTPUT", default_value = "a.out")] 
    output: PathBuf,

    /// Emit LLVM IR instead of object/executable
    #[arg(long, action = clap::ArgAction::SetTrue)]
    emit_ir: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read source file
    let source = std::fs::read_to_string(&args.input)?;

    // Lexing
    let tokens = lexer::lex(&source)?;

    // Parsing
    let program = parser::parse(tokens)?;

    // Semantic analysis
    semantic::analyze(&program)?;

    // Code generation
    if args.emit_ir {
        codegen::emit_llvm_ir(&program, &args.output)?;
    } else {
        codegen::emit_executable(&program, &args.output)?;
    }

    println!("Compiled {} -> {}", args.input.display(), args.output.display());
    Ok(())
}