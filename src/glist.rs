//!  用头尾链表存储表示法建立广义表，输出广义表，求广义表的表头、广义表的表尾和广义表的深度。

use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::{fmt, error};
use std::fmt::{Display, Formatter};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub enum Node<T> {
    Atom(T),
    List(Link<T>, Link<T>), // List(head, tail)
}

impl<T> Node<T> {
    pub fn new_atom(data: T) -> Self {
        Node::Atom(data)
    }
    pub fn new_list(head: Option<Self>, tail: Option<Self>) -> Self {
        Node::List(head.map(|node| Rc::new(RefCell::new(node))),
                   tail.map(|node| Rc::new(RefCell::new(node))))
    }
    pub fn is_atom(&self) -> bool {
        match self {
            Node::Atom(_) => true,
            _ => false
        }
    }
    pub fn is_empty(&self) -> bool {
        assert!(!self.is_atom());
        match self {
            Node::List(hp, _) => hp.is_none(),
            _ => false
        }
    }
    pub fn get_head(&self) -> Link<T> {
        match self {
            Node::Atom(_) => None,
            Node::List(hp, _) => hp.clone()
        }
    }
    pub fn get_tail(&self) -> Link<T> {
        match self {
            Node::Atom(_) => None,
            Node::List(_, tp) => tp.clone()
        }
    }
    pub fn depth(&self) -> usize {
        if self.is_atom() {
            return 0;
        }
        let mut max = 0;
        let mut node = self;
        while let Node::List(Some(hp), tp) = node {
            let dep = hp.as_ref().borrow().depth();
            if dep > max {
                max = dep;
            }
            if let Some(tp) = tp {
                node = unsafe { &*(tp.as_ref().as_ptr()) };
            } else {
                break;
            }
        }
        max + 1
    }
}

impl FromStr for Node<String> {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer::new(s);
        lexer.lex()?;
        let mut stack = Vec::<Self>::new();
        let mut peekable = lexer.tokens.into_iter().peekable();
        loop {
            let token = if let Some(token) = peekable.next() {
                token
            } else {
                break;
            };
            match token {
                Token::OpenParen => {
                    let token = peekable.peek().ok_or(ParserError::new("finished"))?;
                    if *token == Token::CloseParen {
                        peekable.next().unwrap();
                        stack.push(Node::new_list(None, None))
                    }
                }
                Token::Identifier(s) => {
                    let head = Node::new_atom(s);
                    let next = peekable.peek().ok_or(ParserError::new("finished"))?;
                    if *next == Token::CloseParen {
                        peekable.next().unwrap();
                        stack.push(Node::new_list(Some(head), None))
                    } else {
                        stack.push(head);
                    }
                }
                Token::Comma => {}
                Token::CloseParen => {
                    let tail = stack.pop().ok_or(ParserError::new("stack is empty"))?;
                    let head = stack.pop().ok_or(ParserError::new("stack is empty"))?;
                    stack.push(Node::new_list(Some(head), Some(tail)));
                }
            }
        }
        if stack.len() != 1 {
            Err(ParserError::new("more than 1 node in stack"))
        } else {
            stack.pop().ok_or(ParserError::new("stack is empty"))
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Node::Atom(data) => { write!(f, "{}", data)?; }
            Node::List(head, tail) => {
                write!(f, "(")?;
                if let Some(head) = head {
                    write!(f, "{}", head.as_ref().borrow())?;
                };
                if let Some(tail) = tail {
                    write!(f, ",{}", tail.as_ref().borrow())?;
                };
                write!(f, ")")?;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    Comma,
    OpenParen,
    CloseParen,
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct ParserError {
    details: String,
}

impl ParserError {
    fn new(msg: &str) -> ParserError {
        ParserError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

struct Lexer<'a> {
    pub tokens: Vec<Token>,
    buffer: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(buffer: &'a str) -> Lexer<'a> {
        Lexer {
            tokens: Vec::new(),
            buffer: buffer.chars().peekable(),
        }
    }
    /// next fetches the next token and return it, or a LexerError if there are no more.
    fn next(&mut self) -> Result<char, ParserError> {
        match self.buffer.next() {
            Some(char) => Ok(char),
            None => Err(ParserError::new("finished")),
        }
    }

    /// Preview the next character but don't actually increment
    fn preview_next(&mut self) -> Result<char, ParserError> {
        // No need to return a reference, we can return a copy
        match self.buffer.peek() {
            Some(v) => Ok(*v),
            None => Err(ParserError::new("finished")),
        }
    }

    fn lex(&mut self) -> Result<(), ParserError> {
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
            let ch = self.next()?;
            match ch {
                '(' => self.tokens.push(Token::OpenParen),
                ')' => self.tokens.push(Token::CloseParen),
                ',' => self.tokens.push(Token::Comma),
                _ if ch.is_whitespace() => (),
                _ => {
                    let mut buf = ch.to_string();
                    loop {
                        let ch = self.preview_next()?;
                        match ch {
                            '(' | ')' | ',' => break,
                            _ => {
                                buf.push(ch);
                                self.next()?;
                            }
                        }
                    }
                    self.tokens.push(Token::Identifier(buf));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let buf = "(abc, ( d,(( ),(f))))";
        let mut lexer = Lexer::new(buf);
        lexer.lex().expect("lex failed");
        assert_eq!(vec![
            Token::OpenParen,
            Token::Identifier(String::from("abc")),
            Token::Comma,
            Token::OpenParen,
            Token::Identifier(String::from("d")),
            Token::Comma,
            Token::OpenParen,
            Token::OpenParen,
            Token::CloseParen,
            Token::Comma,
            Token::OpenParen,
            Token::Identifier(String::from("f")),
            Token::CloseParen,
            Token::CloseParen,
            Token::CloseParen,
            Token::CloseParen,
        ], lexer.tokens);
    }

    #[test]
    fn test_glist_from_str() {
        let s = "(abc, ( d,(( ),(f))))";
        let node = Node::from_str(s).unwrap();
        assert_eq!("(abc,(d,((),(f))))", format!("{}", node));
    }
}