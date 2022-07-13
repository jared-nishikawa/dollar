use std::fmt;
use crate::scanner;
use crate::scanner::Token;

#[derive(Debug, Clone)]
pub enum Error {
    Eof,
    ParseError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Error::ParseError(s) => s,
            Error::Eof => "EOF",
        };
        write!(f, "{}", t)
    }
}

impl From<scanner::Error> for Error {
    fn from(err: scanner::Error) -> Error {
        Error::ParseError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Node {
    Eof,
    Exp(String),
    DollarExp(String),
}

pub struct Parser {
    pub nodes: Vec<Node>,
    index: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(s: &str) -> Result<Self> {
        let sc = scanner::Scanner::new(s).scan()?;
        let p = Parser {
            index: 0,
            tokens: sc.tokens,
            nodes: Vec::new(),
        };
        Ok(p)
    }

    pub fn parse(mut self) -> Result<Self> {
        loop {
            match self.parse_node() {
                Ok(Node::Eof) => break,
                Ok(node) => self.nodes.push(node),
                Err(e) => return Err(Error::ParseError(e.to_string())),
            }
        }
        Ok(self)
    }

    pub fn parse_node(&mut self) -> Result<Node> {
        let node = match self.peek(0) {
            Token::Eof => Ok(Node::Eof),
            Token::DollarDollar => self.parse_dollardollar(),
            Token::Other(_) | Token::Dollar => self.parse_other(),
        };
        node
    }

    pub fn parse_other(&mut self) -> Result<Node> {
        let mut s_vec: Vec<String> = Vec::new();
        loop {
            match self.peek(0) {
                Token::Other(s) => {
                    s_vec.push(s);
                    self.read();
                }
                Token::Dollar => {
                    s_vec.push("$".to_string());
                    self.read();
                }
                _ => break,
            }
        }
        Ok(Node::Exp(s_vec.join("")))
    }

    pub fn parse_dollardollar(&mut self) -> Result<Node> {
        let d: Node;
        match self.read() {
            Token::DollarDollar => {
                match self.parse_other() {
                    Ok(Node::Exp(s)) => d = Node::DollarExp(s),
                    _ => return Err(Error::ParseError("expected expression".to_string())),
                }
                match self.read() {
                    Token::DollarDollar => (),
                    _ => return Err(Error::ParseError("expected $$".to_string())),
                }

            }
            _ => return Err(Error::ParseError("expected $$".to_string())),
        }
        Ok(d)
    }

    pub fn read(&mut self) -> Token {
        let t = self.peek(0);
        self.index += 1;
        t
    }

    pub fn peek(&mut self, n: usize) -> Token {
        if self.index + n >= self.tokens.len() {
            return Token::Eof;
        }
        self.tokens[self.index + n].clone()
    }

}
