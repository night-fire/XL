use crate::ast::Program;

pub fn generate_code(program: &Program) -> String {
    // TODO: real code generation logic.
    format!("{:#?}", program)
}