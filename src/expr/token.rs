use std::fmt::{Display, Formatter, Result, Debug};

/// Represents a token
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Token {
    pub data: TokenData,
    pub pos: usize,
}

impl Token {
    pub fn new(data: TokenData, pos: usize) -> Self {
        Self {
            data,
            pos,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{}", self.data) }
}

pub struct VecToken(Vec<Token>);

impl Debug for VecToken {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut buffer = String::new();
        for token in &self.0 {
            buffer.push_str(&token.to_string());
        }
        write!(f, "{}", buffer)
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum TokenData {
    Number(f64),
    Operator(Operator),
    Paren(Paren),
}

impl Display for TokenData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.clone() {
            TokenData::Number(num) => write!(f, "{}", num),
            TokenData::Operator(op) => write!(f, "{}", op),
            TokenData::Paren(paren) => write!(f, "{}", paren)
        }
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Operator {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Div => "/"
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Paren {
    /// `(`
    Open,
    /// `)`
    Close,
}

impl Paren {
    /// get the opposite parenthesis
    pub fn opposite_paren(&self) -> Self {
        match self {
            Paren::Open => Paren::Close,
            Paren::Close => Paren::Open
        }
    }
}

impl Display for Paren {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Paren::Open => "(",
                Paren::Close => ")"
            }
        )
    }
}