use inkwell::{context::Context, builder::Builder, module::Module, values::IntValue};

pub struct IR<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> IR<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        IR { context, module, builder }
    }

    pub fn i64_type(&self) -> inkwell::types::IntType<'ctx> {
        self.context.i64_type()
    }

    pub fn const_i64(&self, v: i64) -> IntValue<'ctx> {
        self.i64_type().const_int(v as u64, true)
    }
}