use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::value::{Constant, Double};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    /// Returns the next-strongest precedence. If a stronger precedence does not exist, returns the
    /// strongest precedence.
    const fn strengthen(self) -> Self {
        match self {
            Self::None => Self::Assignment,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call | Self::Primary => Self::Primary,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FunctionRepr {
    Grouping,
    Unary,
    Binary,
    Number,
    Literal,
    String,
}

#[derive(Debug, Clone, Copy)]
struct ParseRule {
    _kind: TokenKind,
    prefix: Option<FunctionRepr>,
    infix: Option<FunctionRepr>,
    precedence: Precedence,
}

#[derive(Debug)]
struct Parser<'src> {
    current: Token<'src>,
    previous: Token<'src>,
    had_error: bool,
    panic_mode: bool,
    rules: [ParseRule; 40],
}

#[rustfmt::skip]
impl<'src> Parser<'src> {
    fn new() -> Self {
        let rules = [
            ParseRule { _kind: TokenKind::LeftParen, prefix: Some(FunctionRepr::Grouping), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::RightParen, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::LeftBrace, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::RightBrace, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Comma, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Dot, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Minus, prefix: Some(FunctionRepr::Unary), infix: Some(FunctionRepr::Binary), precedence: Precedence::Term, },
            ParseRule { _kind: TokenKind::Plus, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Term, },
            ParseRule { _kind: TokenKind::Semicolon, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Slash, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Factor, },
            ParseRule { _kind: TokenKind::Star, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Factor, },
            ParseRule { _kind: TokenKind::Bang, prefix: Some(FunctionRepr::Unary), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::BangEqual, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Equality, },
            ParseRule { _kind: TokenKind::Equal, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::EqualEqual, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Equality, },
            ParseRule { _kind: TokenKind::Greater, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Comparison, },
            ParseRule { _kind: TokenKind::GreaterEqual, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Comparison, },
            ParseRule { _kind: TokenKind::Less, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Comparison, },
            ParseRule { _kind: TokenKind::LessEqual, prefix: None, infix: Some(FunctionRepr::Binary), precedence: Precedence::Comparison, },
            ParseRule { _kind: TokenKind::Identifier, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::String, prefix: Some(FunctionRepr::String), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Number, prefix: Some(FunctionRepr::Number), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::And, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Class, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Else, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::False, prefix: Some(FunctionRepr::Literal), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::For, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Fun, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::If, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Nil, prefix: Some(FunctionRepr::Literal), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Or, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Print, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Return, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Super, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::This, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::True, prefix: Some(FunctionRepr::Literal), infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Var, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::While, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Error, prefix: None, infix: None, precedence: Precedence::None, },
            ParseRule { _kind: TokenKind::Eof, prefix: None, infix: None, precedence: Precedence::None, },
        ];
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_error: false,
            panic_mode: false,
            rules,
        }
    }

    const fn rule(&self, kind: TokenKind) -> ParseRule {
        self.rules[kind as usize]
    }
}

#[derive(Debug)]
pub struct Compiler<'src> {
    source: &'src str,
    parser: Parser<'src>,
    scanner: Scanner<'src>,
    chunk: Chunk<'src>,
}

impl<'src> Compiler<'src> {
    pub fn new() -> Self {
        let source = "";
        let parser = Parser::new();
        let scanner = Scanner::new();
        let chunk = Chunk::new();
        Self {
            source,
            parser,
            scanner,
            chunk,
        }
    }

    pub fn compile(&mut self, source: &'src str) -> Chunk {
        self.source = source;
        self.scanner.update_source(self.source);
        self.advance();
        self.expression();
        self.consume(TokenKind::Eof, "Expect end of expression.");
        self.emit_return();
        if !self.parser.had_error {
            println!("Chunk output:");
            println!("{}", self.chunk);
        }
        std::mem::take(&mut self.chunk)
    }

    fn advance(&mut self) {
        std::mem::swap(&mut self.parser.previous, &mut self.parser.current);
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.kind != TokenKind::Error {
                break;
            }
            self.parser.had_error = true;
            self.parser.panic_mode = true;
            self.error_at(&self.parser.current, "");
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.parser.current.kind == kind {
            self.advance();
            return;
        }
        self.error_at(&self.parser.current, message);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn emit_byte(&mut self, op: OpCode) {
        self.chunk.write(op, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, op1: OpCode, op2: OpCode) {
        self.chunk.write(op1, self.parser.previous.line);
        self.chunk.write(op2, self.parser.previous.line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_constant(&mut self, constant: Constant<'src>) {
        let index = self.chunk.add_constant(constant);
        self.emit_byte(OpCode::Constant(index));
    }

    fn number(&mut self) {
        let value: Double = self
            .parser
            .previous
            .lexeme
            .parse()
            .expect("Lexeme cannot be parsed into value.");
        self.emit_constant(Constant::Number(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_kind = self.parser.previous.kind;
        self.parse_precedence(Precedence::Unary);
        match operator_kind {
            TokenKind::Bang => self.emit_byte(OpCode::Not),
            TokenKind::Minus => self.emit_byte(OpCode::Negate),
            _ => (),
        }
    }

    fn binary(&mut self) {
        let operator_kind = self.parser.previous.kind;
        let rule = self.parser.rule(operator_kind);
        self.parse_precedence(rule.precedence.strengthen());
        match operator_kind {
            TokenKind::Plus => self.emit_byte(OpCode::Add),
            TokenKind::Minus => self.emit_byte(OpCode::Subtract),
            TokenKind::Star => self.emit_byte(OpCode::Multiply),
            TokenKind::Slash => self.emit_byte(OpCode::Divide),
            TokenKind::BangEqual => self.emit_bytes(OpCode::Equal, OpCode::Not),
            TokenKind::EqualEqual => self.emit_byte(OpCode::Equal),
            TokenKind::Greater => self.emit_byte(OpCode::Greater),
            TokenKind::GreaterEqual => self.emit_bytes(OpCode::Less, OpCode::Not),
            TokenKind::Less => self.emit_byte(OpCode::Less),
            TokenKind::LessEqual => self.emit_bytes(OpCode::Greater, OpCode::Not),
            _ => (),
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.kind {
            TokenKind::False => self.emit_byte(OpCode::False),
            TokenKind::Nil => self.emit_byte(OpCode::Nil),
            TokenKind::True => self.emit_byte(OpCode::True),
            _ => (),
        }
    }

    fn string(&mut self) {
        match self.parser.previous.kind {
            TokenKind::String => {
                let str = self.parser.previous.lexeme.trim_matches('"');
                self.emit_constant(Constant::String(str));
            }
            _ => self.error_at(&self.parser.previous, "Expect string constant."),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.parser.rule(self.parser.previous.kind).prefix;
        match prefix_rule {
            Some(FunctionRepr::Grouping) => self.grouping(),
            Some(FunctionRepr::Unary) => self.unary(),
            Some(FunctionRepr::Binary) => self.binary(),
            Some(FunctionRepr::Number) => self.number(),
            Some(FunctionRepr::Literal) => self.literal(),
            Some(FunctionRepr::String) => self.string(),
            None => self.error_at(&self.parser.previous, "Expect prefix expression."),
        }

        while precedence <= self.parser.rule(self.parser.current.kind).precedence {
            self.advance();
            let infix_rule = self.parser.rule(self.parser.previous.kind).infix;
            match infix_rule {
                Some(FunctionRepr::Grouping) => self.grouping(),
                Some(FunctionRepr::Unary) => self.unary(),
                Some(FunctionRepr::Binary) => self.binary(),
                Some(FunctionRepr::Number) => self.number(),
                _ => self.error_at(&self.parser.previous, "Expect infix expression."),
            }
        }
    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        print!("[line {}] Error", token.line);
        if matches!(token.kind, TokenKind::Eof) {
            print!(" at end");
        } else if matches!(token.kind, TokenKind::Error) {
            // Do nothing
        } else {
            print!(" at {}", token.lexeme);
        }

        println!(": {message}");
    }
}
