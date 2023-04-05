use color_eyre::eyre::{self, ensure, eyre};

use crate::BrainfuckInstruction;

pub struct BrainfuckState<Cell, const N: usize> {
    pub instruction_pointer: usize,
    pub data_pointer: usize,
    pub cells: [Cell; N],
}

impl<Cell: Default + Copy, const N: usize> Default for BrainfuckState<Cell, N> {
    fn default() -> Self {
        Self {
            instruction_pointer: 0,
            data_pointer: 0,
            cells: [Cell::default(); N],
        }
    }
}

pub enum EOFBehaviour {
    Value(u8),
    Unchanged,
}

pub enum StepResult {
    Continue,
    Terminated,
}

pub fn brainfuck_step(
    state: &mut BrainfuckState<u8, 30_000>,
    instructions: &[BrainfuckInstruction],
    input: impl FnOnce() -> eyre::Result<Option<u8>>,
    output: impl FnOnce(u8) -> eyre::Result<()>,
    eof_behaviour: EOFBehaviour,
) -> eyre::Result<StepResult> {
    let BrainfuckState {
        instruction_pointer,
        data_pointer,
        cells,
    } = state;

    let cell = &mut cells[*data_pointer];

    match instructions[*instruction_pointer] {
        BrainfuckInstruction::IncrCell => *cell = cell.wrapping_add(1),
        BrainfuckInstruction::DecrCell => *cell = cell.wrapping_sub(1),

        BrainfuckInstruction::Input => {
            match input()? {
                Some(v) => *cell = v,
                None => match eof_behaviour {
                    EOFBehaviour::Value(v) => *cell = v,
                    EOFBehaviour::Unchanged => {}
                },
            };
        }
        BrainfuckInstruction::Output => output(*cell)?,

        BrainfuckInstruction::IncrPtr => {
            *data_pointer += 1;
            ensure!(*data_pointer < cells.len(), "Pointer outside of cells")
        }
        BrainfuckInstruction::DecrPtr => {
            *data_pointer = data_pointer
                .checked_sub(1)
                .ok_or_else(|| eyre!("Pointer underflow"))?
        }
        BrainfuckInstruction::LeftBracket { matching_pos } => {
            if *cell == 0 {
                *instruction_pointer = matching_pos
            }
        }
        BrainfuckInstruction::RightBracket { matching_pos } => {
            if *cell != 0 {
                *instruction_pointer = matching_pos
            }
        }
    }

    *instruction_pointer += 1;

    let result = if *instruction_pointer < instructions.len() {
        StepResult::Continue
    } else {
        StepResult::Terminated
    };
    Ok(result)
}
