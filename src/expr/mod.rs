use crate::expr::token::{Token, TokenData};
use std::{fmt, error};
use std::marker::PhantomData;
use crate::expr::token::Operator;
use crate::expr::lexer::{Lexer, LexerError};
use std::str::FromStr;
use crate::expr::token::Paren;
use std::fmt::{Debug, Formatter, Display};

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
    ty: PhantomData<Ty>,
}

impl<Ty> Expr<Ty> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ty: PhantomData,
        }
    }
}

impl Expr<Postfix> {
    pub fn eval(&self) -> Option<f64> {
        let mut stack: Vec<f64> = vec![];
        for token in self.tokens.iter() {
            match token.data {
                TokenData::Operator(op) => {
                    match op {
                        Operator::Add => {
                            let (x, y) = (stack.pop()?, stack.pop()?);
                            stack.push(y + x);
                        }
                        Operator::Sub => {
                            let (x, y) = (stack.pop()?, stack.pop()?);
                            stack.push(y - x);
                        }
                        Operator::Mul => {
                            let (x, y) = (stack.pop()?, stack.pop()?);
                            stack.push(y * x);
                        }
                        Operator::Div => {
                            let (x, y) = (stack.pop()?, stack.pop()?);
                            stack.push(y / x);
                        }
                    }
                }
                TokenData::Number(num) => {
                    stack.push(num);
                }
                _ => ()
            }
        }
        stack.pop()
    }
}

impl From<Expr<Infix>> for Expr<Postfix> {
    fn from(infix_expr: Expr<Infix>) -> Self {
        let mut post_tokens: Vec<Token> = vec![];
        let mut stack: Vec<Token> = vec![];
        for token in infix_expr.tokens {
            match token.data {
                TokenData::Operator(op) => {
                    match op {
                        Operator::Add | Operator::Sub => {
                            let mut iter = stack.into_iter();
                            stack = vec![];
                            post_tokens.extend(iter.by_ref().rev().take_while(
                                |tk| {
                                    let could_take = tk.data != TokenData::Paren(Paren::Open);
                                    if !could_take {
                                        stack.push(*tk);
                                    }
                                    could_take
                                }
                            ));
                            stack.extend(iter);
                            stack.push(token);
                        }
                        Operator::Mul | Operator::Div => {
                            let lower_levels = [
                                TokenData::Operator(Operator::Add),
                                TokenData::Operator(Operator::Sub),
                                TokenData::Paren(Paren::Open)
                            ];
                            let mut iter = stack.into_iter();
                            stack = vec![];
                            post_tokens.extend(iter.by_ref().rev().take_while(
                                |tk| {
                                    let could_take = !lower_levels.contains(&tk.data);
                                    if !could_take {
                                        stack.push(*tk);
                                    }
                                    could_take
                                }
                            ));
                            stack.extend(iter);
                            stack.push(token);
                        }
                    }
                }
                TokenData::Number(_) => post_tokens.push(token),
                TokenData::Paren(paren) => {
                    match paren {
                        Paren::Open => stack.push(token),
                        Paren::Close => {
                            let mut iter = stack.into_iter();
                            post_tokens.extend(iter.by_ref().rev().take_while(
                                |tk| tk.data != TokenData::Paren(Paren::Open)
                            ));
                            stack = iter.collect();
                        }
                    }
                }
            }
        }
        post_tokens.extend(stack.iter());
        Self::new(post_tokens)
    }
}

impl<Ty> FromStr for Expr<Ty> {
    type Err = LexerError;
    fn from_str(expr: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer::new(expr);
        lexer.lex()?;
        Ok(Self::new(lexer.tokens))
    }
}

impl<Ty> Display for Expr<Ty> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for token in self.tokens.iter() {
            buffer.push_str(&token.to_string());
        }
        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_infix_expr_to_post() {
        let infix_expr: Expr<Infix> = Expr::from_str("1+2*(5-3)").unwrap();
        let postfix_expr: Expr<Postfix> = infix_expr.into();
        assert_eq!("1253-*+", format!("{}", postfix_expr));
    }

    #[test]
    fn test_eval_post_expr() {
        let infix_expr: Expr<Infix> = Expr::from_str("1+2*(5-3)").unwrap();
        let postfix_expr: Expr<Postfix> = infix_expr.into();
        assert_eq!(Some(5.0), postfix_expr.eval());
    }
}