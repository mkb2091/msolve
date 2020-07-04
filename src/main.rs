mod lib;
pub use lib::*;
use std::io::BufRead;
use std::io::Write;

#[derive(Eq, PartialEq)]
enum Mode {
    SolveUnique,
    SolveOne,
    SolveN(usize),
    FindWithSingleSolution,
    FindWithSolution,
    Info,
}

fn main() {
    let mut mode = Mode::Info;
    let mut args = std::env::args_os().skip(1);
    if let Some(arg1) = args.next() {
        if let Some(arg1) = arg1.to_str() {
            mode = match arg1 {
                "solve_unique" => Mode::SolveUnique,
                "solve_one" => Mode::SolveOne,
                "solve_n" => {
                    if let Some(n) = args
                        .next()
                        .and_then(|arg2| arg2.to_str().and_then(|arg| arg.parse::<usize>().ok()))
                    {
                        Mode::SolveN(n)
                    } else {
                        println!("Invalid or missing N argument");
                        return;
                    }
                }
                "info" => Mode::Info,
                "find_with_single_solution" => Mode::FindWithSingleSolution,
                "find_with_solution" => Mode::FindWithSolution,
                _ => {
                    println!("Unknown mode: {}", arg1);
                    return;
                }
            }
        } else {
            println!("Argument is not valid UTF-8");
            return;
        }
    }

    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::with_capacity(82);
    let stdout = std::io::stdout();
    let mut output_handle = stdout.lock();
    let mut info = [0; 3];
    while let Ok(result) = input.read_line(&mut buffer) {
        if result == 0 {
            break;
        }
        let sudoku = msolve::Sudoku::from(&buffer);
        match mode {
            Mode::SolveUnique => {
                if let Some(solution) = sudoku.solve_unique() {
                    let _ = output_handle.write_all(&solution.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }
            }
            Mode::SolveOne => {
                if let Some(solution) = sudoku.solve_one() {
                    let _ = output_handle.write_all(&solution.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }
            }
            Mode::SolveN(n) => {
                for solution in sudoku.iter().take(n) {
                    let _ = output_handle.write_all(&solution.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }
            }
            Mode::Info => {
                info[sudoku.count_solutions(2)] += 1;
            }
            Mode::FindWithSingleSolution => {
                if sudoku.has_single_solution() {
                    let _ = output_handle.write_all(&sudoku.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }
            }
            Mode::FindWithSolution => {
                if sudoku.has_solution() {
                    let _ = output_handle.write_all(&sudoku.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }
            }
        }
        buffer.clear();
    }
    if mode == Mode::Info {
        println!(
            "0 Solutions: {}, 1 Solution: {}, 2+ Solutions: {}",
            info[0], info[1], info[2]
        );
    }
}
