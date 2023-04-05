use std::fmt::Display;

pub mod interpreter;
pub mod parser;

#[derive(Debug)]
pub enum BrainfuckInstruction {
    IncrCell,
    DecrCell,

    Input,
    Output,

    IncrPtr,
    DecrPtr,
    LeftBracket { matching_pos: usize },
    RightBracket { matching_pos: usize },
}

pub struct BrainfuckProgram {
    pub instructions: Vec<BrainfuckInstruction>,
    pub spans: Vec<DocumentSpan>,
}

#[derive(Clone, Copy)]
pub struct DocumentSpan {
    pub column: usize,
    pub row: usize,
}

impl Display for DocumentSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.column, self.row)
    }
}
