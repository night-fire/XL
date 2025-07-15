use crate::ast::Program;
use crate::error::XLError;

pub enum Phase {
    Parsing,
    Analysis,
    Transformation,
    Codegen,
}

pub trait Pass {
    fn name(&self) -> &'static str;
    fn phase(&self) -> Phase;
    fn run(&self, program: &mut Program) -> Result<(), XLError>;
}

pub struct PassManager {
    passes: Vec<Box<dyn Pass>>,
}

impl PassManager {
    pub fn new() -> Self {
        PassManager { passes: Vec::new() }
    }

    pub fn add_pass<P: Pass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&self, program: &mut Program) -> Result<(), XLError> {
        for p in &self.passes {
            p.run(program)?;
        }
        Ok(())
    }
}

/// Convenience wrapper converting a free function `fn(&mut Program) -> Result<()>` to a Pass
pub struct FnPass<F>
where
    F: Fn(&mut Program) -> Result<(), XLError>,
{
    func: F,
    name: &'static str,
    phase: Phase,
}

impl<F> FnPass<F>
where
    F: Fn(&mut Program) -> Result<(), XLError>,
{
    pub fn new(name: &'static str, phase: Phase, func: F) -> Self {
        FnPass { func, name, phase }
    }
}

impl<F> Pass for FnPass<F>
where
    F: Fn(&mut Program) -> Result<(), XLError>,
{
    fn name(&self) -> &'static str { self.name }
    fn phase(&self) -> Phase { self.phase }
    fn run(&self, program: &mut Program) -> Result<(), XLError> { (self.func)(program) }
}