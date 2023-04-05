use color_eyre::eyre::{self, ensure, eyre, Context};
use std::{
    fmt::Display,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use brainfuck_interpreter::{
    interpreter::{brainfuck_step, BrainfuckState, EOFBehaviour, StepResult},
    parser::ProgramParser,
    BrainfuckProgram, DocumentSpan,
};

struct ContextualizedSpan<'a> {
    path: &'a Path,
    span: DocumentSpan,
}

impl Display for ContextualizedSpan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.path.display(), self.span)
    }
}

fn run_program(program: &BrainfuckProgram, path: &Path) -> eyre::Result<()> {
    use std::io::{stdin, stdout, Write};

    let BrainfuckProgram {
        instructions,
        spans,
    } = program;

    let mut stdin = stdin().lock().bytes();
    let mut stdout = stdout();

    let mut state = BrainfuckState::default();

    loop {
        let input = || stdin.next().transpose().map_err(|e| e.into());

        let output = |c| {
            stdout.write(&[c]).map_err(|e| e.into()).and_then(|s| {
                ensure!(s == 1, "Failed to output");
                Ok(())
            })
        };

        let context = ContextualizedSpan {
            path,
            span: spans[state.instruction_pointer],
        };
        let result = brainfuck_step(
            &mut state,
            instructions,
            input,
            output,
            EOFBehaviour::Value(0),
        )
        .wrap_err_with(|| format!("Error at {context}"))?;

        match result {
            StepResult::Continue => continue,
            StepResult::Terminated => break,
        }
    }

    Ok(())
}

fn parse_args() -> eyre::Result<PathBuf> {
    use std::env::args_os;

    let mut args = args_os();

    args.next();
    let program_file_path = args.next().ok_or_else(|| eyre!("No program file passed"))?;
    Ok(program_file_path.into())
}

fn parse_file(file: File, path: &Path) -> eyre::Result<BrainfuckProgram> {
    let mut parser = ProgramParser::default();

    let mut newline_count: usize = 0;
    let mut last_newline = 0;
    for (i, b) in file.bytes().enumerate() {
        let b = b?;
        let span = DocumentSpan {
            column: newline_count + 1,
            row: (i + 1) - last_newline,
        };

        let context = ContextualizedSpan { path, span };

        parser = parser
            .input_byte(b, span)
            .wrap_err_with(|| format!("Parsing error at {context}"))?;
        if b == b'\n' {
            newline_count += 1;
            last_newline = i;
        }
    }

    parser.finalize().wrap_err("Invalid Brainfuck program")
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let program_file_path = parse_args().wrap_err("Failed to get program file path from args")?;

    let program_file = File::open(&program_file_path).wrap_err("Failed to open program file")?;

    let program = parse_file(program_file, &program_file_path).wrap_err("Failed to parse file")?;

    run_program(&program, &program_file_path).wrap_err("Failed to run program")?;

    Ok(())
}
