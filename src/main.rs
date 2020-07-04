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
    CountSolutions(usize),
    Info,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let mode = if let Some(arg1) = args.next() {
        if let Some(arg1) = arg1.to_str() {
            match arg1 {
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
                "count_solutions" => {
                    if let Some(n) = args
                        .next()
                        .and_then(|arg2| arg2.to_str().and_then(|arg| arg.parse::<usize>().ok()))
                    {
                        Mode::CountSolutions(n)
                    } else {
                        println!("Invalid or missing N argument, where N is the maximum number of solutions to count");
                        return;
                    }
                }
                _ => {
                    println!("Unknown mode: {}", arg1);
                    return;
                }
            }
        } else {
            println!("Argument is not valid UTF-8");
            return;
        }
    } else {
        println!(
            "
Usage:
	msolve {{mode}} < input.txt

	Or

	other_program | msolve {{mode}}


Modes:

	solve_unique: returns solution for each uniquely solvable sudoku in input

	solve_one: returns first solution found for each sudoku in input, order is not guaranteed to be constant

	solve_n {{N}}: returns first N solution found for each sudoku in input, order is not guaranteed to be constant

	find_with_single_solution: returns all sudokus with a single unique solution

	find_with_solution: returns all sudokus with at least one solution

	count_solutions {{N}}: returns each sudoku with the number of solutions up to a maximum of N in the format {{count}};{{sudoku}}

	info: returns the number of puzzles with no solution, 1 solution and 2+ solutions
			"
        );
        return;
    };

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
        if let Ok(sudoku) = buffer.parse::<msolve::Sudoku>() {
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
                Mode::CountSolutions(n) => {
                    let count = sudoku.count_solutions(n);

                    let _ = output_handle.write_all(&count.to_string().as_bytes());
                    let _ = output_handle.write_all(b";");
                    let _ = output_handle.write_all(&buffer.as_bytes());
                }
                Mode::Info => {
                    info[sudoku.count_solutions(2)] += 1;
                }
            }
        } else {
            match mode {
                Mode::Info => {
                    info[0] += 1;
                }
                Mode::CountSolutions(_) => {
                    let _ = output_handle.write_all(b"0;");
                    let _ = output_handle.write_all(&buffer.as_bytes());
                }
                _ => {}
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
