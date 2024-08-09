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
                OpCode::Add => {
                    let b = self.stack.pop().ok_or(Error::Compiler)?;
                    let a = self.stack.pop().ok_or(Error::Compiler)?;
                    self.stack.push(a + b);
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().ok_or(Error::Compiler)?;
                    let a = self.stack.pop().ok_or(Error::Compiler)?;
                    self.stack.push(a - b);
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().ok_or(Error::Compiler)?;
                    let a = self.stack.pop().ok_or(Error::Compiler)?;
                    self.stack.push(a * b);
                }
                OpCode::Divide => {
                    let b = self.stack.pop().ok_or(Error::Compiler)?;
                    let a = self.stack.pop().ok_or(Error::Compiler)?;
                    self.stack.push(a / b);
                }
                OpCode::Negate => {
                    let arg = self.stack.pop().ok_or(Error::Compiler)?;
                    self.stack.push(-arg);
                }
                OpCode::Return => {
                    let ret = self.stack.pop().ok_or(Error::Compiler)?;
                    println!("{ret}");
                }
            }
        }
        Ok(())
    }
}
