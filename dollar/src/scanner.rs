use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    Eof,
    ScanError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Error::ScanError(s) => s,
            Error::Eof => "EOF",
        };
        write!(f, "{}", t)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    Dollar,
    DollarDollar,
    Other(String),
}

pub struct Scanner {
    pub tokens: Vec<Token>,
    index: usize,
    input: Vec<char>,
}

impl Scanner {
    pub fn new(s: &str) -> Self {
        Scanner {
            tokens: Vec::new(),
            index: 0,
            input: s.chars().collect::<Vec<_>>(),
        }
    }

    pub fn scan(mut self) -> Result<Self> {
        loop {
            match self.scan_token() {
                Ok(Token::Eof) => break,
                Ok(tok) => self.tokens.push(tok),
                Err(e) => return Err(Error::ScanError(e.to_string())),
            }
        }
        Ok(self)
    }

    pub fn scan_token(&mut self) -> Result<Token> {
        let token = match self.peek(0) {
            '\0' => Ok(Token::Eof),
            '$' => {
                self.read();
                match self.peek(0) {
                    '$' => {
                        self.read();
                        Ok(Token::DollarDollar)
                    }
                    _ => Ok(Token::Dollar),
                }
            }
            _ => self.read_any(),
        };
        token
    }

    pub fn read_any(&mut self) -> Result<Token> {
        let mut char_vec: Vec<char> = Vec::new();
        loop {
            match self.peek(0) {
                '\0' | '$' => break,
                '\\' => {
                    self.read();
                    char_vec.push(self.read());
                }
                c => {
                    char_vec.push(c);
                    self.read();
                }
            }
        }
        let s: String = char_vec.into_iter().collect();
        Ok(Token::Other(s))
    }

    pub fn read(&mut self) -> char {
        let c = self.peek(0);
        self.index += 1;
        c
    }

    pub fn peek(&mut self, n: usize) -> char {
        if self.index + n >= self.input.len() {
            return '\0';
        }
        self.input[self.index + n]
    }
}
