use anyhow::{bail, Result};

use super::ast::*;
use super::lexer::{Token, TokenKind};
use super::types::Type;

pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut p = Parser { tokens, pos: 0 };
    p.parse_program()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            None
        } else {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        }
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    fn expect_keyword(&mut self, expected: TokenKind) -> Result<()> {
        match self.bump() {
            Some(tok) if tok.kind == expected => Ok(()),
            _ => bail!("Expected {:?}", expected),
        }
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut items = Vec::new();
        while self.peek().is_some() {
            items.push(Item::Function(self.parse_function()?));
        }
        Ok(items)
    }

    fn parse_function(&mut self) -> Result<Function> {
        self.expect_keyword(TokenKind::Function)?;
        let name = self.expect_ident()?;
        self.expect_keyword(TokenKind::LParen)?;
        let mut params = Vec::new();
        if self.peek_kind() != Some(&TokenKind::RParen) {
            loop {
                let param_name = self.expect_ident()?;
                self.expect_keyword(TokenKind::Colon)?;
                let ty = self.parse_type()?;
                params.push(Param { name: param_name, ty });
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.bump();
                } else {
                    break;
                }
            }
        }
        self.expect_keyword(TokenKind::RParen)?;
        self.expect_keyword(TokenKind::Arrow)?;
        let ret_type = self.parse_type()?;
        self.expect_keyword(TokenKind::LBrace)?;
        let mut body = Vec::new();
        while self.peek_kind() != Some(&TokenKind::RBrace) {
            body.push(self.parse_stmt()?);
        }
        self.expect_keyword(TokenKind::RBrace)?;
        Ok(Function { name, params, ret_type, body })
    }

    fn parse_type(&mut self) -> Result<Type> {
        match self.bump() {
            Some(tok) if tok.kind == TokenKind::Ident => Ok(Type::I32), // Only i32 supported for now
            _ => bail!("Unknown type"),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.peek_kind() {
            Some(&TokenKind::Return) => {
                self.bump();
                let expr = self.parse_expr()?;
                self.expect_keyword(TokenKind::Semi)?;
                Ok(Stmt::Return(expr))
            }
            _ => bail!("Unsupported statement"),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut node = self.parse_factor()?;
        while let Some(kind) = self.peek_kind() {
            match kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let op = if kind == &TokenKind::Plus { BinOp::Add } else { BinOp::Sub };
                    self.bump();
                    let rhs = self.parse_factor()?;
                    node = Expr::Binary { left: Box::new(node), op, right: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        let mut node = self.parse_primary()?;
        while let Some(kind) = self.peek_kind() {
            match kind {
                TokenKind::Star | TokenKind::Slash => {
                    let op = if kind == &TokenKind::Star { BinOp::Mul } else { BinOp::Div };
                    self.bump();
                    let rhs = self.parse_primary()?;
                    node = Expr::Binary { left: Box::new(node), op, right: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.bump() {
            Some(tok) => match tok.kind {
                TokenKind::Int(val) => Ok(Expr::Int(val)),
                TokenKind::Ident => {
                    let ident_name = tok.text;
                    if self.peek_kind() == Some(&TokenKind::LParen) {
                        self.expect_keyword(TokenKind::LParen)?;
                        let mut args = Vec::new();
                        if self.peek_kind() != Some(&TokenKind::RParen) {
                            loop {
                                args.push(self.parse_expr()?);
                                if self.peek_kind() == Some(&TokenKind::Comma) {
                                    self.bump();
                                } else {
                                    break;
                                }
                            }
                        }
                        self.expect_keyword(TokenKind::RParen)?;
                        Ok(Expr::Call { func: ident_name, args })
                    } else {
                        Ok(Expr::Ident(ident_name))
                    }
                }
                TokenKind::True => Ok(Expr::Bool(true)),
                TokenKind::False => Ok(Expr::Bool(false)),
                _ => bail!("Unexpected token in expression"),
            },
            None => bail!("Unexpected EOF in expression"),
        }
    }

    // Helpers
    fn expect_ident(&mut self) -> Result<String> {
        match self.bump() {
            Some(tok) if tok.kind == TokenKind::Ident => Ok(tok.text),
            _ => bail!("Expected identifier"),
        }
    }

    fn prev_ident(&self) -> Result<String> {
        if self.pos == 0 { bail!("No previous token")?; }
        let tok = &self.tokens[self.pos - 1];
        if tok.kind == TokenKind::Ident {
            Ok(tok.text.clone())
        } else {
            bail!("Previous token is not identifier")
        }
    }
}