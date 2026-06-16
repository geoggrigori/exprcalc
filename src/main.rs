//! Command-line front end for the `exprcalc` expression evaluator.
//!
//! Usage:
//!
//! - With arguments: the arguments are joined with spaces, evaluated as a
//!   single expression, and the result is printed. On error the message is
//!   written to stderr and the process exits with status `1`.
//! - Without arguments: a simple REPL reads expressions line by line from
//!   stdin, printing each result (or error) until EOF.

use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use exprcalc::eval;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        repl()
    } else {
        let input = args.join(" ");
        match eval(&input) {
            Ok(value) => {
                println!("{value}");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("error: {err}");
                ExitCode::FAILURE
            }
        }
    }
}

/// Run the read-eval-print loop until end of input.
fn repl() -> ExitCode {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let prompt = |out: &mut io::Stdout| {
        let _ = write!(out, "> ");
        let _ = out.flush();
    };

    prompt(&mut stdout);
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(err) => {
                eprintln!("error: failed to read input: {err}");
                return ExitCode::FAILURE;
            }
        };

        let trimmed = line.trim();
        if !trimmed.is_empty() {
            match eval(trimmed) {
                Ok(value) => println!("{value}"),
                Err(err) => eprintln!("error: {err}"),
            }
        }
        prompt(&mut stdout);
    }

    // Emit a trailing newline so the shell prompt starts on a fresh line.
    println!();
    ExitCode::SUCCESS
}
