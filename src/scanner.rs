use crate::error::{Error, RloxResult};
use crate::value::Line;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: Line,
}

impl<'a> Token<'a> {
    fn new(token_type: TokenType, lexeme: &'a str, line: Line) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: Line,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        self.source = &self.source[self.start..];
        let mut iter = self.source.chars().peekable();
        while let Some(c) = iter.next() {
            self.current += 1;
            match c {
                '\t' | ' ' | '\r' => continue,
                '\n' => {
                    self.line += 1;
                    continue;
                }
                '/' if iter.peek().is_some_and(|c| *c == '/') => {
                    while iter.next().is_some_and(|c| c != '\n') {
                        self.current += 1;
                    }
                    self.line += 1;
                }
                '(' => return self.create_token(TokenType::LeftParen),
                ')' => return self.create_token(TokenType::RightParen),
                '{' => return self.create_token(TokenType::LeftBrace),
                '}' => return self.create_token(TokenType::RightBrace),
                ';' => return self.create_token(TokenType::Semicolon),
                ',' => return self.create_token(TokenType::Comma),
                '.' => return self.create_token(TokenType::Dot),
                '-' => return self.create_token(TokenType::Minus),
                '+' => return self.create_token(TokenType::Plus),
                '/' => return self.create_token(TokenType::Slash),
                '*' => return self.create_token(TokenType::Star),
                '!' if iter.peek().is_some_and(|c| *c == '=') => {
                    iter.next();
                    self.current += 1;
                    return self.create_token(TokenType::BangEqual);
                }
                '!' => return self.create_token(TokenType::Bang),
                '=' if iter.peek().is_some_and(|c| *c == '=') => {
                    iter.next();
                    self.current += 1;
                    return self.create_token(TokenType::EqualEqual);
                }
                '=' => return self.create_token(TokenType::Equal),
                '<' if iter.peek().is_some_and(|c| *c == '=') => {
                    self.current += 1;
                    return self.create_token(TokenType::LessEqual);
                }
                '<' => return self.create_token(TokenType::Less),
                '>' if iter.peek().is_some_and(|c| *c == '=') => {
                    self.current += 1;
                    return self.create_token(TokenType::GreaterEqual);
                }
                '>' => return self.create_token(TokenType::Greater),
                '"' => {
                    for c in iter.by_ref() {
                        self.current += 1;
                        if c == '\n' {
                            self.line += 1;
                        }
                        if c == '"' {
                            return self.create_token(TokenType::String)
                        }
                    }
                    return self.error_token("Unterminated string")
                }
                '0'..='9' => {
                    while iter.peek().is_some_and(|c| c.is_ascii_digit() || *c == '.') {
                        let _ = iter.next();
                        self.current += 1;
                    }
                    return self.create_token(TokenType::Number)
                } 
                'a'..='z' | 'A'..='Z' | '_' => {
                    while iter.peek().is_some_and(|c| c.is_alphanumeric() || *c == '_') {
                        let _ = iter.next();
                        self.current += 1;
                    }
                    return self.create_token(TokenType::Identifier)
                },
                _ => todo!(),
            }
        }
        Token::new(TokenType::EOF, "", self.line)
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        )
    }

    fn error_token(&self, message: &'a str) -> Token {
        Token::new(TokenType::Error, message, self.line)
    }
}
