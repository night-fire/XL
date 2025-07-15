use super::super::*;

pub struct Mem2Reg;
impl SSAPass for Mem2Reg {
    fn name(&self) -> &'static str { "mem2reg" }
    fn run(&self, _func: &mut Function) -> bool {
        // Placeholder: real algorithm pending
        false
    }
}