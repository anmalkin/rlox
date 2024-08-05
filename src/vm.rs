use crate::chunk::{Chunk, OpCode};
use crate::error::{Error, RloxResult};
use crate::value::Value;
use crate::compiler::compile;

#[derive(Debug)]
pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        let chunk = None;
        let ip = 0;
        let stack = Vec::with_capacity(256);
        Self { chunk, ip, stack }
    }

    pub fn interpret(&mut self, source: String) -> RloxResult {
        compile(source)
    }

    fn run(&mut self) -> RloxResult {
        let chunk = self.chunk.ok_or(Error::Compiler)?;
        for instruction in &chunk.code {
            // for value in &self.stack {
            //     println!("[{value:#?}]");
            // }
            self.ip += 1;
            match instruction {
                OpCode::Constant(index) => {
                    let constant = chunk.constant(*index);
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
