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

// ===============================================================
// New generic combinator-based parsing library (experimental)
// ===============================================================

pub mod comb {
    use super::*;

    pub type ParseResult<'a, T> = Result<(&'a str, T), &'a str>;

    pub trait Parser<'a, T> {
        fn parse(&self, input: &'a str) -> ParseResult<'a, T>;

        fn map<U, F>(self, f: F) -> Map<Self, F, T, U>
        where
            Self: Sized,
            F: Fn(T) -> U,
        {
            Map { parser: self, func: f, _phantom: std::marker::PhantomData }
        }

        fn then<P2, U>(self, p2: P2) -> Then<Self, P2, T, U>
        where
            Self: Sized,
            P2: Parser<'a, U>,
        {
            Then { first: self, second: p2, _phantom: std::marker::PhantomData }
        }

        fn or<P2>(self, p2: P2) -> Or<Self, P2, T>
        where
            Self: Sized,
            P2: Parser<'a, T>,
        {
            Or { left: self, right: p2, _phantom: std::marker::PhantomData }
        }
    }

    impl<'a, F, T> Parser<'a, T> for F
    where
        F: Fn(&'a str) -> ParseResult<'a, T>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
            self(input)
        }
    }

    pub struct Map<P, F, A, B> {
        parser: P,
        func: F,
        _phantom: std::marker::PhantomData<(A, B)>,
    }

    impl<'a, P, F, A, B> Parser<'a, B> for Map<P, F, A, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, B> {
            let (rest, val) = self.parser.parse(input)?;
            Ok((rest, (self.func)(val)))
        }
    }

    pub struct Then<P1, P2, A, B> {
        first: P1,
        second: P2,
        _phantom: std::marker::PhantomData<(A, B)>,
    }

    impl<'a, P1, P2, A, B> Parser<'a, (A, B)> for Then<P1, P2, A, B>
    where
        P1: Parser<'a, A>,
        P2: Parser<'a, B>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, (A, B)> {
            let (rest, a) = self.first.parse(input)?;
            let (rest2, b) = self.second.parse(rest)?;
            Ok((rest2, (a, b)))
        }
    }

    pub struct Or<P1, P2, T> {
        left: P1,
        right: P2,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<'a, P1, P2, T> Parser<'a, T> for Or<P1, P2, T>
    where
        P1: Parser<'a, T>,
        P2: Parser<'a, T>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
            self.left.parse(input).or_else(|_| self.right.parse(input))
        }
    }

    // --- Primitive parsers -------------------------------------------------

    pub fn charp<'a>(c: char) -> impl Parser<'a, char> {
        move |input: &'a str| {
            if let Some(first) = input.chars().next() {
                if first == c {
                    let rest = &input[first.len_utf8()..];
                    return Ok((rest, first));
                }
            }
            Err(input)
        }
    }

    pub fn digit1<'a>() -> impl Parser<'a, &'a str> {
        move |input: &'a str| {
            let mut idx = 0;
            for ch in input.chars() {
                if ch.is_ascii_digit() {
                    idx += ch.len_utf8();
                } else {
                    break;
                }
            }
            if idx == 0 {
                return Err(input);
            }
            Ok((&input[idx..], &input[..idx]))
        }
    }
}