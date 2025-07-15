use crate::compiler::ir::{Instruction, Module};
use crate::compiler::types::Type;

pub fn generate_rust(module: &Module) -> String {
    let mut out = String::new();
    out.push_str("// Auto-generated Rust code from XL compiler (via IR)\n");
    for fun in &module.functions {
        gen_function(fun, &mut out);
    }
    // Ensure main exists
    if !module.functions.iter().any(|f| f.name == "main") {
        out.push_str("fn main() {}\n");
    }
    out
}

fn gen_function(fun: &crate::compiler::ir::Function, out: &mut String) {
    out.push_str("fn ");
    out.push_str(&fun.name);
    out.push('(');
    for (i, ty) in fun.params.iter().enumerate() {
        if i > 0 { out.push(','); }
        out.push_str(&format!("arg{}: {}", i, ty));
    }
    out.push(')');
    // Assume return type i32 or void for now
    // TODO store ret type in IR
    out.push_str(" -> i32 {");
    out.push('\n');

    // Map value ids to local names
    let mut value_names = Vec::new();

    for block in &fun.blocks {
        for instr in &block.instrs {
            match instr {
                Instruction::ConstInt { dest, value } => {
                    let name = format!("tmp{}", dest);
                    ensure_size(&mut value_names, *dest);
                    value_names[*dest] = name.clone();
                    out.push_str(&format!("    let {} = {}i32;\n", name, value));
                }
                Instruction::Add { dest, lhs, rhs } => {
                    let name = format!("tmp{}", dest);
                    ensure_size(&mut value_names, *dest);
                    value_names[*dest] = name.clone();
                    out.push_str(&format!(
                        "    let {} = {} + {};\n",
                        name,
                        value_names[*lhs].clone(),
                        value_names[*rhs].clone()
                    ));
                }
                Instruction::Sub { dest, lhs, rhs } => {
                    let name = format!("tmp{}", dest);
                    ensure_size(&mut value_names, *dest);
                    value_names[*dest] = name.clone();
                    out.push_str(&format!(
                        "    let {} = {} - {};\n",
                        name,
                        value_names[*lhs].clone(),
                        value_names[*rhs].clone()
                    ));
                }
                Instruction::Mul { dest, lhs, rhs } => {
                    let name = format!("tmp{}", dest);
                    ensure_size(&mut value_names, *dest);
                    value_names[*dest] = name.clone();
                    out.push_str(&format!(
                        "    let {} = {} * {};\n",
                        name,
                        value_names[*lhs].clone(),
                        value_names[*rhs].clone()
                    ));
                }
                Instruction::Div { dest, lhs, rhs } => {
                    let name = format!("tmp{}", dest);
                    ensure_size(&mut value_names, *dest);
                    value_names[*dest] = name.clone();
                    out.push_str(&format!(
                        "    let {} = {} / {};\n",
                        name,
                        value_names[*lhs].clone(),
                        value_names[*rhs].clone()
                    ));
                }
                Instruction::Return { value } => {
                    if let Some(v) = value {
                        out.push_str(&format!("    return {};\n", value_names[*v]));
                    } else {
                        out.push_str("    return;\n");
                    }
                }
            }
        }
    }

    out.push_str("}\n\n");
}

fn ensure_size(vec: &mut Vec<String>, idx: usize) {
    if vec.len() <= idx {
        vec.resize(idx + 1, String::new());
    }
}