use crate::chunk::Chunk;
use crate::error::Error;
use crate::scanner::{Scanner, Token, TokenKind};

#[derive(Debug)]
struct Parser<'src> {
    current: Token<'src>,
    previous: Token<'src>,
    had_error: bool,
    panic_mode: bool,
}

impl<'src> Parser<'src> {
    fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_error: false,
            panic_mode: false,
        }
    }
}

#[derive(Debug)]
pub struct Compiler<'src> {
    source: &'src str,
    parser: Parser<'src>,
    scanner: Scanner<'src>,
}

impl<'src> Compiler<'src> {
    pub fn new() -> Self {
        let source = "";
        let parser = Parser::new();
        let scanner = Scanner::new();
        Self {
            source,
            parser,
            scanner,
        }
    }

    pub fn compile(&mut self, source: &'src str) -> Result<Chunk, Error> {
        self.source = source;
        self.scanner.update_source(self.source);
        self.scanner.scan_token();
        todo!()
    }

    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        let mut next_token = self.scanner.scan_token();
        while matches!(self.parser.current.kind, TokenKind::Error) {
            next_token = self.scanner.scan_token();
            Compiler::error_at(&self.parser.current, "");
            self.parser.had_error = true;
        }
        self.parser.current = next_token;
    }

    pub fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.parser.current.kind != kind {
            Compiler::error_at(&self.parser.current, message);
        }
        self.advance();
    }

    fn error_at(token: &Token, message: &str) {
        eprint!("[line {}] Error", token.line);
        if matches!(token.kind, TokenKind::Eof) {
            eprint!(" at end");
        } else if matches!(token.kind, TokenKind::Error) {
            // Do nothing
        } else {
            eprint!(" at {}", token.lexeme);
        }

        eprintln!(": {message}");
    }
}
