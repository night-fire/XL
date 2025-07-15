use crate::ast::*;
use crate::ast::RewriteRule;
use crate::error::XLError;
use crate::lexer::Spanned;
use crate::token::Token;

pub type Tokens = Vec<Spanned<Token>>;

pub fn parse(tokens: Tokens) -> Result<Program, XLError> {
    let mut p = Parser { tokens, pos: 0 };
    p.parse_program()
}

struct Parser {
    tokens: Tokens,
    pos: usize,
}

impl Parser {
    fn parse_program(&mut self) -> Result<Program, XLError> {
        let mut functions = Vec::new();
        while !self.is_eof() {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function, XLError> {
        self.expect_keyword(Token::Fn)?;
        let name = self.expect_ident()?;
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                let param_name = self.expect_ident()?;
                self.expect(Token::Colon)?;
                let ty = self.parse_type()?;
                params.push(Param { name: param_name, ty });
                if self.check(Token::RParen) {
                    break;
                }
                self.expect(Token::Comma)?;
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::Arrow)?;
        let return_type = self.parse_type()?;
        let body = self.parse_block()?;
        Ok(Function { name, params, return_type, body })
    }

    fn parse_block(&mut self) -> Result<Block, XLError> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();
        while !self.check(Token::RBrace) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(Block { stmts })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, XLError> {
        if self.match_keyword(Token::Let) {
            let name = self.expect_ident()?;
            let mut ty = None;
            if self.match_token(Token::Colon) {
                ty = Some(self.parse_type()?);
            }
            self.expect(Token::Equal)?;
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Let { name, ty, value: expr })
        } else if self.match_keyword(Token::Return) {
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Return(expr))
        } else {
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Expr(expr))
        }
    }

    fn parse_type(&mut self) -> Result<Type, XLError> {
        if let Some(tok) = self.peek() {
            if let Some(ty) = Type::from_keyword(&tok.token) {
                self.advance();
                Ok(ty)
            } else {
                Err(self.error(format!("expected type, found {:?}", tok.token)))
            }
        } else {
            Err(self.error("unexpected EOF while parsing type".into()))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, XLError> {
        self.parse_binary_expr(0)
    }

    fn parse_primary(&mut self) -> Result<Expr, XLError> {
        match self.peek_token() {
            Some(Token::IntLiteral(value)) => {
                let v = *value;
                self.advance();
                Ok(Expr::Int(v))
            }
            Some(Token::StringLiteral(ref s)) => {
                let v = s.clone();
                self.advance();
                Ok(Expr::String(v))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Some(Token::RewriteKw) => {
                return self.parse_rewrite_expr();
            }
            Some(Token::Match) => {
                return self.parse_match_expr();
            }
            Some(Token::Ident(ref name)) => {
                let ident = name.clone();
                self.advance();
                if self.match_token(Token::LParen) {
                    // Parse call
                    let mut args = Vec::new();
                    if !self.check(Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.check(Token::RParen) { break; }
                            self.expect(Token::Comma)?;
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { callee: ident, args })
                } else {
                    Ok(Expr::Ident(ident))
                }
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            other => Err(self.error(format!("unexpected token {:?} in expression", other))),
        }
    }

    fn parse_match_expr(&mut self) -> Result<Expr, XLError> {
        self.expect_keyword(Token::Match)?;
        let value = self.parse_expr()?;
        self.expect(Token::LBrace)?;
        let mut arms = Vec::new();
        while !self.check(Token::RBrace) {
            let pat = self.parse_pattern()?;
            self.expect(Token::FatArrow)?;
            let arm_expr = self.parse_expr()?;
            // optional comma
            self.match_token(Token::Comma);
            arms.push((pat, arm_expr));
        }
        self.expect(Token::RBrace)?;
        Ok(Expr::Match {
            value: Box::new(value),
            arms,
        })
    }

    fn parse_rewrite_expr(&mut self) -> Result<Expr, XLError> {
        self.expect_keyword(Token::RewriteKw)?;
        self.expect(Token::LBrace)?;
        let mut rules = Vec::new();
        while !self.check(Token::RBrace) {
            let pat = self.parse_pattern()?;
            let guard = if self.match_keyword(Token::If) {
                // parse guard expression
                Some(self.parse_expr()?)
            } else {
                None
            };
            self.expect(Token::FatArrow)?;
            let repl = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            rules.push(RewriteRule { pattern: pat, guard, replacement: repl });
        }
        self.expect(Token::RBrace)?;
        // After `rewrite { rules }` expect target expression in parentheses
        self.expect(Token::LParen)?;
        let target = self.parse_expr()?;
        self.expect(Token::RParen)?;
        Ok(Expr::Rewrite { rules, target: Box::new(target) })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, XLError> {
        match self.peek_token() {
            Some(Token::IntLiteral(v)) => {
                let val = *v;
                self.advance();
                Ok(Pattern::Int(val))
            }
            Some(Token::StringLiteral(ref s)) => {
                let txt = s.clone();
                self.advance();
                Ok(Pattern::String(txt))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Pattern::Bool(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(Pattern::Bool(false))
            }
            Some(Token::Underscore) => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            Some(Token::Dollar) => {
                self.advance();
                let name = self.expect_ident()?;
                Ok(Pattern::Capture(name))
            }
            Some(Token::Ident(ref name)) => {
                let id = name.clone();
                self.advance();
                Ok(Pattern::Ident(id))
            }
            other => Err(self.error(format!("unexpected token {:?} in pattern", other))),
        }
    }

    // Precedence climbing for binary expr
    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, XLError> {
        let mut left = self.parse_primary()?;
        while let Some(op_tok) = self.peek_token() {
            let (op, prec) = match op_tok {
                Token::Plus => (BinaryOp::Add, 1),
                Token::Minus => (BinaryOp::Sub, 1),
                Token::Star => (BinaryOp::Mul, 2),
                Token::Slash => (BinaryOp::Div, 2),
                _ => break,
            };
            if prec < min_prec { break; }
            self.advance(); // consume operator
            let mut right = self.parse_binary_expr(prec + 1)?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // Helpers
    fn peek(&self) -> Option<&Spanned<Token>> {
        self.tokens.get(self.pos)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.peek().map(|s| &s.token)
    }

    fn check(&self, kind: Token) -> bool {
        self.peek_token().map_or(false, |t| *t == kind)
    }

    fn match_keyword(&mut self, kind: Token) -> bool {
        if self.check(kind.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_token(&mut self, kind: Token) -> bool {
        if self.check(kind.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_keyword(&mut self, kind: Token) -> Result<(), XLError> {
        if self.match_keyword(kind.clone()) {
            Ok(())
        } else {
            Err(self.error(format!("expected {:?}", kind)))
        }
    }

    fn expect(&mut self, kind: Token) -> Result<(), XLError> {
        if self.match_token(kind.clone()) {
            Ok(())
        } else {
            Err(self.error(format!("expected {:?}", kind)))
        }
    }

    fn expect_ident(&mut self) -> Result<String, XLError> {
        match self.peek_token() {
            Some(Token::Ident(ref s)) => {
                let name = s.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(self.error("expected identifier".into())),
        }
    }

    fn advance(&mut self) {
        if !self.is_eof() {
            self.pos += 1;
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn error(&self, msg: String) -> XLError {
        XLError::ParseError(msg)
    }
}