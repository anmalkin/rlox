use std::fmt::Display;

use crate::value::{Constant, Line};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Constant(usize),
    Nil,
    True,
    False,
    Pop,
    GetGlobal(usize),
    DefineGlobal(usize),
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,
    Return,
}

#[derive(Debug)]
pub struct Chunk<'src> {
    pub code: Vec<OpCode>,
    constants: Vec<Constant<'src>>,
    lines: Vec<Line>,
}

impl<'src> Chunk<'src> {
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

    pub fn constant(&self, index: usize) -> Constant {
        self.constants[index]
    }

    pub fn add_constant(&mut self, constant: Constant<'src>) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
}

impl<'src> Display for Chunk<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr_line = 0;
        for (i, op) in self.code.iter().enumerate() {
            let mut output: String;
            let line = self.lines[i];
            if line == curr_line {
                output = ("| ").to_string();
            } else {
                curr_line = line;
                output = format!("{line} ");
            }
            output.push_str(format!("{op:?}").as_str());
            if let OpCode::Constant(index) = op {
                let constant = self.constant(*index);
                output.push_str(format!("    {constant:?}").as_str());
            }
            writeln!(f, "{output}")?;
        }
        Ok(())
    }
}

impl<'src> Default for Chunk<'src> {
    fn default() -> Self {
        let code = Vec::new();
        let constants = Vec::new();
        let lines = Vec::new();
        Self {
            code,
            constants,
            lines,
        }
    }
}
