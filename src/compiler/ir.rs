use std::fmt;

use super::types::Type;

pub type ValueId = usize;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Type>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub instrs: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    ConstInt { dest: ValueId, value: i64 },
    Add { dest: ValueId, lhs: ValueId, rhs: ValueId },
    Sub { dest: ValueId, lhs: ValueId, rhs: ValueId },
    Mul { dest: ValueId, lhs: ValueId, rhs: ValueId },
    Div { dest: ValueId, lhs: ValueId, rhs: ValueId },
    Return { value: Option<ValueId> },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::ConstInt { dest, value } => write!(f, "%{} = const {}", dest, value),
            Instruction::Add { dest, lhs, rhs } => write!(f, "%{} = add %{}, %{}", dest, lhs, rhs),
            Instruction::Sub { dest, lhs, rhs } => write!(f, "%{} = sub %{}, %{}", dest, lhs, rhs),
            Instruction::Mul { dest, lhs, rhs } => write!(f, "%{} = mul %{}, %{}", dest, lhs, rhs),
            Instruction::Div { dest, lhs, rhs } => write!(f, "%{} = div %{}, %{}", dest, lhs, rhs),
            Instruction::Return { value } => match value {
                Some(v) => write!(f, "ret %{}", v),
                None => write!(f, "ret"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self { functions: Vec::new() }
    }
}