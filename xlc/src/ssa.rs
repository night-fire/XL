//! Minimal SSA intermediate representation.

use std::collections::HashMap;
pub mod passes;
pub mod lower;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

#[derive(Debug, Clone)]
pub enum ValueKind {
    Const(i64),
    Add(ValueId, ValueId),
    Sub(ValueId, ValueId),
    Mul(ValueId, ValueId),
    Div(ValueId, ValueId),
    Phi(Vec<(BlockId, ValueId)>),
}

#[derive(Debug, Clone)]
pub struct Value {
    pub id: ValueId,
    pub kind: ValueKind,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub values: Vec<ValueId>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Ret(ValueId),
    Br(BlockId),
    CondBr { cond: ValueId, then_bb: BlockId, else_bb: BlockId },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub blocks: HashMap<BlockId, BasicBlock>,
    pub entry: BlockId,
    pub values: HashMap<ValueId, Value>,
    next_val: u32,
    next_bb: u32,
}

impl Function {
    pub fn new(name: String) -> Self {
        let entry = BlockId(0);
        Self {
            name,
            blocks: HashMap::new(),
            entry,
            values: HashMap::new(),
            next_val: 0,
            next_bb: 1,
        }
    }

    pub fn new_value(&mut self, kind: ValueKind) -> ValueId {
        let id = ValueId(self.next_val);
        self.next_val += 1;
        self.values.insert(id, Value { id, kind });
        id
    }

    pub fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_bb);
        self.next_bb += 1;
        id
    }
}

// ---------- Optimization pass trait ----------

pub trait SSAPass {
    fn name(&self) -> &'static str;
    fn run(&self, func: &mut Function) -> bool; // returns *changed?*
}

pub struct PassManagerSSA {
    passes: Vec<Box<dyn SSAPass>>,
}

impl PassManagerSSA {
    pub fn new() -> Self { Self { passes: Vec::new() } }
    pub fn add<P: SSAPass + 'static>(&mut self, p: P) { self.passes.push(Box::new(p)); }
    pub fn run(&self, func: &mut Function) {
        let mut changed = true;
        while changed {
            changed = false;
            for p in &self.passes {
                changed |= p.run(func);
            }
        }
    }
    pub fn opt_level_1() -> Self {
        let mut pm = PassManagerSSA::new();
        pm.add(crate::ssa::passes::mem2reg::Mem2Reg);
        pm.add(crate::ssa::passes::ConstPropPass);
        pm.add(crate::ssa::passes::DcePass);
        pm
    }
}

/// Example constant-propagation pass (stub)
pub struct ConstProp;
impl SSAPass for ConstProp {
    fn name(&self) -> &'static str { "constprop" }
    fn run(&self, _func: &mut Function) -> bool {
        // TODO: implement constant propagation over SSA graph.
        false
    }
}