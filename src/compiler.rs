use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::value::{Constant, Double};

#[rustfmt::skip]
const RULES: [ParseRule; 40] = [
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
    ParseRule { _kind: TokenKind::Identifier, prefix: Some(FunctionRepr::Variable), infix: None, precedence: Precedence::None, },
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
    Variable,
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

    const fn rule(kind: TokenKind) -> ParseRule {
        RULES[kind as usize]
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
        while !self.match_token(TokenKind::Eof) {
            self.declaration();
        }
        self.consume(TokenKind::Eof, "Expect end of expression");
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

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.parser.current.kind != kind { return false }
        self.advance();
        true
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.match_token(TokenKind::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::Nil);
        }
        self.consume(TokenKind::Semicolon, "Expect ';' after variable declaration.");
        self.define_variable(global);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ; after expression.");
        self.emit_byte(OpCode::Pop);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ; after value.");
        self.emit_byte(OpCode::Print);
    }

    fn synchronize(&mut self) {
        self.parser.panic_mode = false;

        while self.parser.current.kind != TokenKind::Eof {
            if self.parser.previous.kind == TokenKind::Semicolon {
                return;
            }
            match self.parser.current.kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Return => return,
                _ => (),
            }
            self.advance();
        }
    }

    fn declaration(&mut self) {
        if self.match_token(TokenKind::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self) {
        if self.match_token(TokenKind::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
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
        let rule = Parser::rule(operator_kind);
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

    fn named_variable(&mut self, name: &'src str) {
        let arg = self.identifier_constant(name);
        self.emit_byte(OpCode::GetGlobal(arg));
    }

    fn variable(&mut self) {
        self.named_variable(self.parser.previous.lexeme);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = Parser::rule(self.parser.previous.kind).prefix;
        match prefix_rule {
            Some(FunctionRepr::Grouping) => self.grouping(),
            Some(FunctionRepr::Unary) => self.unary(),
            Some(FunctionRepr::Binary) => self.binary(),
            Some(FunctionRepr::Number) => self.number(),
            Some(FunctionRepr::Literal) => self.literal(),
            Some(FunctionRepr::String) => self.string(),
            Some(FunctionRepr::Variable) => self.variable(),
            None => self.error_at(&self.parser.previous, "Expect prefix expression."),
        }

        while precedence <= Parser::rule(self.parser.current.kind).precedence {
            self.advance();
            let infix_rule = Parser::rule(self.parser.previous.kind).infix;
            match infix_rule {
                Some(FunctionRepr::Grouping) => self.grouping(),
                Some(FunctionRepr::Unary) => self.unary(),
                Some(FunctionRepr::Binary) => self.binary(),
                Some(FunctionRepr::Number) => self.number(),
                _ => self.error_at(&self.parser.previous, "Expect infix expression."),
            }
        }
    }

    fn identifier_constant(&mut self, name: &'src str) -> usize {
        self.chunk.add_constant(Constant::String(name))
    }

    fn parse_variable(&mut self, err_msg: &str) -> usize {
        self.consume(TokenKind::Identifier, err_msg);
        self.identifier_constant(self.parser.previous.lexeme)
    }

    fn define_variable(&mut self, global: usize) {
        self.emit_byte(OpCode::DefineGlobal(global));
    }

    fn error_at(&self, token: &Token, err_msg: &str) {
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

        println!(": {err_msg}");
    }
}
