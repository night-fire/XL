use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod compiler;

/// XL Compiler (xlc)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the source .xl file
    input: PathBuf,
    /// Path for the produced executable (default: a.out)
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    compiler::compile(&args.input, &args.output)
}