pub mod lexer;
pub mod parser;
pub mod ast;
pub mod codegen;

use std::path::Path;

use anyhow::{Context, Result};
use tempfile::NamedTempFile;

/// Compile an XL source file to a native executable.
/// 
/// Pipeline:
/// 1. Lex & parse to AST
/// 2. Generate Rust source code
/// 3. Invoke `rustc` to produce standalone binary
pub fn compile(input: &Path, output: &Path) -> Result<()> {
    // Read source
    let source = std::fs::read_to_string(input)
        .with_context(|| format!("Unable to read source file {}", input.display()))?;

    // Lex & parse
    let tokens = lexer::lex(&source)?;
    let ast = parser::parse(tokens)?;

    // Codegen to Rust
    let rust_source = codegen::generate_rust(&ast);

    // Write temp Rust file
    let mut tmp = NamedTempFile::new().context("Failed to create temp file")?;
    std::io::Write::write_all(&mut tmp, rust_source.as_bytes())?;
    let tmp_path = tmp.into_temp_path();

    // Call rustc
    let status = std::process::Command::new("rustc")
        .arg(&*tmp_path)
        .arg("-o")
        .arg(output)
        .status()
        .context("Failed to spawn rustc")?;

    if !status.success() {
        anyhow::bail!("rustc failed with status {}", status);
    }
    Ok(())
}