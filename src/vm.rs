use crate::chunk::{Chunk, OpCode};
use crate::compiler::Compiler;
use crate::error::{Error, RloxResult};
use crate::value::Value;

#[derive(Debug)]
pub struct VM<'src> {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    compiler: Compiler<'src>,
}

impl<'src> VM<'src> {
    pub fn new() -> Self {
        let chunk = Chunk::new();
        let ip = 0;
        let stack = Vec::with_capacity(256);
        let compiler = Compiler::new();
        Self {
            chunk,
            ip,
            stack,
            compiler,
        }
    }

    pub fn interpret(&'src mut self, source: &'src str) -> RloxResult {
        self.chunk = self.compiler.compile(source);
        self.ip = 0;
        for instruction in &self.chunk.code {
            self.ip += 1;
            match instruction {
                OpCode::Constant(index) => {
                    let constant = self.chunk.constant(*index);
                    self.stack.push(constant);
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Add => {
                    let Value::Number(b) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    let Value::Number(a) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    self.stack.push(Value::Number(a + b));
                }
                OpCode::Subtract => {
                    let Value::Number(b) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    let Value::Number(a) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    self.stack.push(Value::Number(a - b));
                }
                OpCode::Multiply => {
                    let Value::Number(b) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    let Value::Number(a) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    self.stack.push(Value::Number(a * b));
                }
                OpCode::Divide => {
                    let Value::Number(b) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    let Value::Number(a) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    self.stack.push(Value::Number(a / b));
                }
                OpCode::Not => {
                    match self.stack.pop().ok_or(Error::Compiler)? {
                        Value::Bool(false) | Value::Nil => self.stack.push(Value::Bool(true)),
                        Value::Number(_) | Value::Bool(true) => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::Negate => {
                    let Value::Number(a) = self.stack.pop().ok_or(Error::Compiler)? else {
                        return Err(Error::Runtime);
                    };
                    self.stack.push(Value::Number(-a));
                }
                OpCode::Return => {
                    let ret = self.stack.pop().ok_or(Error::Compiler)?;
                    match ret {
                        Value::Bool(bool) => println!("{bool}"),
                        Value::Nil => println!(),
                        Value::Number(num) => println!("{num}"),
                    }
                }
            }
        }
        Ok(())
    }
}
