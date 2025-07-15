use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod token;
mod lexer;
mod parser;
mod ast;
mod codegen;
mod error;

use lexer::lex;
use parser::Parser as XlParser;
use codegen::generate_code;

#[derive(Parser, Debug)]
#[command(name = "xlc", author, version, about = "XL Compiler")] 
struct Cli {
    /// Input source file (.xl)
    input: PathBuf,
    /// Output file name. If not supplied, prints to stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let source = fs::read_to_string(&cli.input)
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", cli.input.display(), e))?;

    // Lexing
    let tokens = lex(&source);

    // Parsing
    let mut parser = XlParser::new(&tokens);
    let program = parser.parse_program()?;

    // Code generation (placeholder)
    let output = generate_code(&program);

    match &cli.output {
        Some(out_path) => {
            fs::write(out_path, output)?;
        }
        None => {
            println!("{}", output);
        }
    }

    Ok(())
}