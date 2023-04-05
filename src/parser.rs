use color_eyre::eyre::{self, ensure, eyre};

use crate::{BrainfuckInstruction, BrainfuckProgram, DocumentSpan};

#[derive(Default)]
pub struct ProgramParser {
    bracket_stack: Vec<usize>,
    instructions: Vec<BrainfuckInstruction>,
    instruction_spans: Vec<DocumentSpan>,
}

impl ProgramParser {
    pub fn input_byte(mut self, c: u8, span: DocumentSpan) -> eyre::Result<Self> {
        let Self {
            bracket_stack,
            instructions,
            instruction_spans,
        } = &mut self;

        let pos = instructions.len();

        let instruction = match c {
            b'+' => BrainfuckInstruction::IncrCell,
            b'-' => BrainfuckInstruction::DecrCell,

            b',' => BrainfuckInstruction::Input,
            b'.' => BrainfuckInstruction::Output,

            b'>' => BrainfuckInstruction::IncrPtr,
            b'<' => BrainfuckInstruction::DecrPtr,
            b'[' => {
                bracket_stack.push(pos);
                BrainfuckInstruction::LeftBracket { matching_pos: 0 }
            }
            b']' => {
                let left_pos = bracket_stack
                    .pop()
                    .ok_or_else(|| eyre!("Program contains a ] without a corresponding ["))?;

                ensure!(
                    matches!(
                        instructions[left_pos],
                        BrainfuckInstruction::LeftBracket { matching_pos: 0 },
                    ),
                    "Assertion error! Previous instruction wasn't correctly set"
                );
                instructions[left_pos] = BrainfuckInstruction::LeftBracket { matching_pos: pos };
                BrainfuckInstruction::RightBracket {
                    matching_pos: left_pos,
                }
            }
            _ => return Ok(self),
        };
        instructions.push(instruction);
        instruction_spans.push(span);

        Ok(self)
    }

    pub fn finalize(self) -> eyre::Result<BrainfuckProgram> {
        ensure!(
            self.bracket_stack.is_empty(),
            "The program contains a [ without a corresponding ]"
        );

        Ok(BrainfuckProgram {
            instructions: self.instructions,
            spans: self.instruction_spans,
        })
    }
}
