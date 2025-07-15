use anyhow::Result;

use crate::ast::Program;
use crate::token::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        // TODO: real parsing logic. For now, simply return all tokens wrapped.
        Ok(Program {
            tokens: self.tokens.to_vec(),
        })
    }
}