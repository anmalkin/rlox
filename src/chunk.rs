#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    Return,
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        let code = Vec::new();
        let constants = Vec::new();
        Self {code, constants}
    }

    pub fn write_chunk(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
}
