#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use std::fmt::Display;

use crate::value::Value;

type Line = u16;

#[derive(Debug)]
pub enum OpCode {
    Constant(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<Line>,
}

impl Chunk {
    pub fn new() -> Self {
        let code = Vec::new();
        let constants = Vec::new();
        let lines = Vec::new();
        Self {
            code,
            constants,
            lines,
        }
    }

    pub fn write(&mut self, op: OpCode, line: Line) {
        self.code.push(op);
        self.lines.push(line);
        assert_eq!(self.code.len(), self.lines.len());
    }

    pub fn constant(&self, index: usize) -> Value {
        self.constants[index]
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr_line = 0;
        for (i, op) in self.code.iter().enumerate() {
            let line = self.lines[i];
            if line == curr_line {
                let output = format!("  | {op:?}");
                writeln!(f, "{output}")?;
            } else {
                curr_line = line;
                let output = format!("{line} {op:?}");
                writeln!(f, "{output}")?;
            }
        }
        Ok(())
    }
}

