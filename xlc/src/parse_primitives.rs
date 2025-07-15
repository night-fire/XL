#![allow(dead_code)]

/// A very small, hand-rolled parser combinator core.
/// This module demonstrates how XL could expose primitives
/// useful when writing new compilers.

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    /// Peek next character without consuming.
    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Consume and return next character.
    pub fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    /// Parse an ASCII digit sequence and return it as `i64`.
    pub fn parse_int(&mut self) -> Option<i64> {
        let start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.consume();
        }
        if self.pos > start {
            self.input[start..self.pos].parse().ok()
        } else {
            None
        }
    }

    /// Parse and consume an expected character.
    pub fn expect_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.consume();
            true
        } else {
            false
        }
    }
}