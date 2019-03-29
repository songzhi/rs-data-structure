use super::token::{Token, TokenData, Operator, Paren};
use std::{fmt, error, str::{FromStr, Chars}, iter::Peekable};


/// An error that occurred during lexing or compiling of the source input.
#[derive(Debug, Clone)]
pub struct LexerError {
    details: String,
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for LexerError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    pos: usize,
    buffer: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Lexer<'a> {
        Lexer {
            tokens: Vec::new(),
            pos: 0,
            buffer: buffer.chars().peekable(),
        }
    }

    /// Push tokens onto the token queue
    fn push_token(&mut self, tk: TokenData) {
        self.tokens.push(Token::new(tk, self.pos))
    }

    /// Push a operator token
    fn push_operator(&mut self, op: Operator) {
        self.push_token(TokenData::Operator(op));
    }

    /// next fetches the next token and return it, or a LexerError if there are no more.
    fn next(&mut self) -> Result<char, LexerError> {
        match self.buffer.next() {
            Some(char) => Ok(char),
            None => Err(LexerError::new("finished")),
        }
    }

    /// Preview the next character but don't actually increment
    fn preview_next(&mut self) -> Result<char, LexerError> {
        // No need to return a reference, we can return a copy
        match self.buffer.peek() {
            Some(v) => Ok(*v),
            None => Err(LexerError::new("finished")),
        }
    }

    /// next_is compares the character passed in to the next character, if they match true is returned and the buffer is incremented
    fn next_is(&mut self, peek: char) -> Result<bool, LexerError> {
        let result = self.preview_next()? == peek;
        if result {
            self.buffer.next();
        }
        Ok(result)
    }

    pub fn lex(&mut self) -> Result<(), LexerError> {
        loop {
            // Check if we've reached the end
            match self.preview_next() {
                Ok(_) => (), // If there are still characters, carry on
                Err(e) => {
                    if e.details == "finished" {
                        // If there are no more characters left in the Chars iterator, we should just return
                        return Ok(());
                    } else {
                        return Err(e);
                    }
                }
            }
            self.pos += 1;
            let ch = self.next()?;
            match ch {
                '0' => {
                    let mut buf = String::new();
                    let num = if self.next_is('x')? {
                        loop {
                            let ch = self.preview_next()?;
                            match ch {
                                ch if ch.is_digit(16) => {
                                    buf.push(self.next()?);
                                }
                                _ => break,
                            }
                        }
                        u64::from_str_radix(&buf, 16).unwrap()
                    } else {
                        loop {
                            let ch = self.preview_next()?;
                            match ch {
                                ch if ch.is_digit(8) => {
                                    buf.push(ch);
                                    self.next()?;
                                }
                                '8' | '9' | '.' => {
                                    buf.push(ch);
                                    self.next()?;
                                }
                                _ => break,
                            }
                        }
                        u64::from_str_radix(&buf, 8).unwrap()
                    };
                    self.push_token(TokenData::Number(num as f64))
                }
                _ if ch.is_digit(10) => {
                    let mut buf = ch.to_string();
                    loop {
                        let ch = self.preview_next()?;
                        match ch {
                            '.' => {
                                buf.push(self.next()?);
                            }
                            _ if ch.is_digit(10) => {
                                buf.push(self.next()?);
                            }
                            _ => break,
                        }
                    }
                    self.push_token(TokenData::Number(
                        f64::from_str(&buf).map_err(|_| LexerError::new("Number parsing failed"))?
                    ))
                }
                '(' => self.push_token(TokenData::Paren(Paren::Open)),
                ')' => self.push_token(TokenData::Paren(Paren::Close)),
                '+' => self.push_operator(Operator::Add),
                '-' => self.push_operator(Operator::Sub),
                '*' => self.push_operator(Operator::Mul),
                '/' => self.push_operator(Operator::Div),
                '\r' => {
                    self.pos = 0;
                }
                ' ' => (),
                ch => panic!(
                    "{}: Unexpected '{}'",
                    self.pos, ch
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let expr = "1 +2.3 *(0x2/5 -2)";
        let mut lexer = Lexer::new(expr);
        assert!(lexer.lex().is_ok());
        assert_eq!(1, lexer.tokens[0].pos);
        assert_eq!(3, lexer.tokens[1].pos);
        assert_eq!(4, lexer.tokens[2].pos);
    }
}
