use crate::value::Line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
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
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: &'src str,
    pub line: Line,
}

impl<'src> Token<'src> {
    const fn new(token_type: TokenKind, lexeme: &'src str, line: Line) -> Self {
        Self {
            kind: token_type,
            lexeme,
            line,
        }
    }
}

impl<'src> Default for Token<'src> {
    fn default() -> Self {
        Self {
            kind: TokenKind::Eof,
            lexeme: "",
            line: 0,
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: Line,
}

impl<'src> Scanner<'src> {
    pub const fn new() -> Self {
        Self {
            source: "",
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn update_source(&mut self, source: &'src str) {
        self.source = source;
    }

    pub fn scan_token(&mut self) -> Token<'src> {
        self.start = self.current;
        let source = &self.source[self.start..];
        let mut iter = source.chars().peekable();
        while let Some(c) = iter.next() {
            self.current += 1;
            match c {
                '\t' | ' ' | '\r' => {
                    self.start += 1;
                    continue;
                }
                '\n' => {
                    self.start += 1;
                    self.line += 1;
                    continue;
                }
                '/' if iter.peek().is_some_and(|c| *c == '/') => {
                    while iter.next().is_some_and(|c| c != '\n') {
                        self.current += 1;
                    }
                    self.line += 1;
                }
                '(' => return self.create_token(TokenKind::LeftParen),
                ')' => return self.create_token(TokenKind::RightParen),
                '{' => return self.create_token(TokenKind::LeftBrace),
                '}' => return self.create_token(TokenKind::RightBrace),
                ';' => return self.create_token(TokenKind::Semicolon),
                ',' => return self.create_token(TokenKind::Comma),
                '.' => return self.create_token(TokenKind::Dot),
                '-' => return self.create_token(TokenKind::Minus),
                '+' => return self.create_token(TokenKind::Plus),
                '/' => return self.create_token(TokenKind::Slash),
                '*' => return self.create_token(TokenKind::Star),
                '!' if iter.peek().is_some_and(|c| *c == '=') => {
                    iter.next();
                    self.current += 1;
                    return self.create_token(TokenKind::BangEqual);
                }
                '!' => return self.create_token(TokenKind::Bang),
                '=' if iter.peek().is_some_and(|c| *c == '=') => {
                    iter.next();
                    self.current += 1;
                    return self.create_token(TokenKind::EqualEqual);
                }
                '=' => return self.create_token(TokenKind::Equal),
                '<' if iter.peek().is_some_and(|c| *c == '=') => {
                    self.current += 1;
                    return self.create_token(TokenKind::LessEqual);
                }
                '<' => return self.create_token(TokenKind::Less),
                '>' if iter.peek().is_some_and(|c| *c == '=') => {
                    self.current += 1;
                    return self.create_token(TokenKind::GreaterEqual);
                }
                '>' => return self.create_token(TokenKind::Greater),
                '"' => {
                    for c in iter.by_ref() {
                        self.current += 1;
                        if c == '\n' {
                            self.line += 1;
                        }
                        if c == '"' {
                            return self.create_token(TokenKind::String);
                        }
                    }
                    return self.error_token("Unterminated string");
                }
                '0'..='9' => {
                    while iter.peek().is_some_and(|c| c.is_ascii_digit() || *c == '.') {
                        let _ = iter.next();
                        self.current += 1;
                    }
                    return self.create_token(TokenKind::Number);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    while iter
                        .peek()
                        .is_some_and(|c| c.is_alphanumeric() || *c == '_')
                    {
                        let _ = iter.next();
                        self.current += 1;
                    }
                    return self.create_token(self.identifier_type());
                }
                _ => return self.error_token("Invalid syntax"),
            }
        }
        Token::new(TokenKind::Eof, "", self.line)
    }

    fn create_token(&self, token_type: TokenKind) -> Token<'src> {
        Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        )
    }

    fn identifier_type(&self) -> TokenKind {
        let source = &self.source[self.start..self.current];
        let mut chars = source.chars().peekable();
        match dbg!(chars.next()).expect("No character found in identifier.") {
            'a' => self.check_keyword("nd", TokenKind::And),
            'c' => self.check_keyword("lass", TokenKind::Class),
            'e' => self.check_keyword("lse", TokenKind::Else),
            'i' => self.check_keyword("f", TokenKind::If),
            'n' => self.check_keyword("il", TokenKind::Nil),
            'o' => self.check_keyword("r", TokenKind::Or),
            'p' => self.check_keyword("rint", TokenKind::Print),
            'r' => self.check_keyword("eturn", TokenKind::Return),
            's' => self.check_keyword("uper", TokenKind::Super),
            'v' => self.check_keyword("ar", TokenKind::Var),
            'w' => self.check_keyword("hile", TokenKind::While),
            'f' => match dbg!(chars.peek()) {
                Some('a') => self.check_keyword("alse", TokenKind::False),
                Some('o') => self.check_keyword("or", TokenKind::For),
                Some('u') => self.check_keyword("un", TokenKind::Fun),
                _ => TokenKind::Identifier,
            },
            't' => match chars.peek() {
                Some('h') => self.check_keyword("his", TokenKind::This),
                Some('r') => self.check_keyword("rue", TokenKind::True),
                _ => TokenKind::Identifier,
            },
            _ => TokenKind::Identifier,
        }
    }

    fn check_keyword(&self, rest: &'src str, token_type: TokenKind) -> TokenKind {
        let source = &mut self.source[self.start+1..].chars();
        for a in rest.chars() {
            if !source.next().is_some_and(|c| c == a) {
                return TokenKind::Identifier;
            }
        }
        token_type
    }

    const fn error_token(&self, message: &'src str) -> Token<'src> {
        Token::new(TokenKind::Error, message, self.line)
    }
}
