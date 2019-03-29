use crate::expr::token::{Token, TokenData};
use std::{fmt, error};
use std::marker::PhantomData;
use crate::expr::token::Operator;
use crate::expr::lexer::{Lexer, LexerError};
use std::str::FromStr;
use crate::expr::token::TokenData::Paren;

pub mod lexer;
pub mod token;

/// An error that occurred during lexing or compiling of the source input.
#[derive(Debug, Clone)]
pub struct ExprError {
    details: String,
}

impl ExprError {
    fn new(msg: &str) -> ExprError {
        ExprError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for ExprError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Copy, Debug, Clone)]
pub enum Prefix {}

#[derive(Copy, Debug, Clone)]
pub enum Infix {}

#[derive(Copy, Debug, Clone)]
pub enum Postfix {}

pub trait ExprType {
    fn is_prefix() -> bool { false }
    fn is_infix() -> bool { false }
    fn is_postfix() -> bool { false }
}

impl ExprType for Prefix {
    fn is_prefix() -> bool { true }
}

impl ExprType for Infix {
    fn is_infix() -> bool { true }
}

impl ExprType for Postfix {
    fn is_postfix() -> bool { true }
}

pub struct Expr<Ty = Infix> {
    tokens: Vec<Token>,
    val: Option<f64>,
    ty: PhantomData<Ty>,
}

impl<Ty> Expr<Ty> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            val: None,
            ty: PhantomData,
        }
    }
}


impl<Ty> FromStr for Expr<Ty> {
    type Err = LexerError;
    fn from_str(expr: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer::new(expr);
        lexer.lex()?;
        Ok(Self {
            tokens: lexer.tokens,
            val: None,
            ty: PhantomData,
        })
    }
}



pub fn infix_expr_to_post(tokens: impl Iterator<Item=char>) -> String {
    let mut post_expr = String::new();
    let mut stack = vec![];
    for token in tokens {
        match token {
            '+' | '-' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        stack.push(op);
                        break;
                    }
                    post_expr.push(op);
                }
                stack.push(token);
            }
            '(' => stack.push(token),
            '*' | '/' => {
                while let Some(op) = stack.pop() {
                    if "+-(".contains(op) {
                        stack.push(op);
                        break;
                    }
                    post_expr.push(op);
                }
                stack.push(token);
            }
            ')' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        break;
                    }
                    post_expr.push(op);
                }
            }
            _ => post_expr.push(token)
        }
    }
    while let Some(op) = stack.pop() {
        post_expr.push(op);
    }
    post_expr
}

pub fn eval_post_expr(expr: impl Iterator<Item=char>) -> Option<i64> {
    let mut stack: Vec<i64> = vec![];
    for ch in expr {
        match ch {
            '+' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y + x);
            }
            '-' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y - x);
            }
            '*' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y * x);
            }
            '/' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y / x);
            }
            '0'..='9' => {
                stack.push(ch.to_digit(10)?.into());
            }
            _ => { return None; }
        }
    }
    stack.pop()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_infix_expr_to_post() {
        let infix_expr = "a+b*c+(d*e+f)*g";
        let postfix_expr = infix_expr_to_post(infix_expr.chars().into_iter());
        assert_eq!("abc*+de*f+g*+", postfix_expr);
    }

    #[test]
    fn test_eval_post_expr() {
        let expr = "1+2*(5-3)";
        let post_expr = infix_expr_to_post(expr.chars().into_iter());
        assert_eq!(Some(5), eval_post_expr(post_expr.chars().into_iter()));
    }
}