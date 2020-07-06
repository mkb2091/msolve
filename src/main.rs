mod lib;
pub use lib::*;

#[cfg(feature = "cli")]
mod cli {

    use std::io::BufRead;
    use std::io::Write;

    use clap::Clap;

    #[derive(Clap, Copy, Clone)]
    #[clap(version = "1.0")]
    struct Opts {
        #[clap(subcommand)]
        mode: Mode,
    }

    #[derive(Clap, Copy, Clone)]
    enum Mode {
        Solve(Solve),
        Select(Select),
        Difficulty(Difficulty),
        CountSolutions(CountSolutions),
        Generate(Generate),
        Info,
    }

    #[derive(Clap, Copy, Clone)]
    struct Solve {
        #[clap(short, long)]
        unique: bool,
        #[clap(short, long, default_value = "1")]
        count: usize,
    }
    #[derive(Clap, Copy, Clone)]
    struct Select {
        #[clap(short, long)]
        verify_uniqueness: bool,
        #[clap(short, long)]
        invert: bool,
    }
    #[derive(Clap, Copy, Clone)]
    struct Difficulty {
        #[clap(short, long)]
        verify_uniqueness: bool,
    }

    #[derive(Clap, Copy, Clone)]
    struct CountSolutions {
        n: usize,
    }

    #[derive(Clap, Copy, Clone)]
    struct Generate {
        cells_to_remove: usize,
    }

    pub fn main() {
        let opts: Opts = Opts::parse();

        let stdin = std::io::stdin();
        let mut input = stdin.lock();
        let mut buffer = String::with_capacity(82);
        let stdout = std::io::stdout();
        let mut output_handle = stdout.lock();
        let mut info = [0; 3];
        let mut rng = rand::thread_rng();
        while let Ok(result) = input.read_line(&mut buffer) {
            if result == 0 {
                break;
            }
            let sudoku = buffer.parse::<msolve::Sudoku>().unwrap();
            match opts.mode {
                Mode::Solve(solve) => {
                    if solve.unique {
                        if let Some(solution) = sudoku.solve_unique() {
                            let _ = output_handle.write_all(&solution.to_bytes());
                            let _ = output_handle.write_all(b"\n");
                        }
                    } else {
                        for solution in sudoku.iter().take(solve.count) {
                            let _ = output_handle.write_all(&solution.to_bytes());
                            let _ = output_handle.write_all(b"\n");
                        }
                    }
                }
                Mode::Select(select) => {
                    let mut does_match = if select.verify_uniqueness {
                        sudoku.has_single_solution()
                    } else {
                        sudoku.has_solution()
                    };
                    if select.invert {
                        does_match = !does_match;
                    }
                    if does_match {
                        let _ = output_handle.write_all(&sudoku.to_bytes());
                        let _ = output_handle.write_all(b"\n");
                    }
                }
                Mode::Difficulty(difficulty) => {
                    let steps = if difficulty.verify_uniqueness {
                        sudoku.solve_unique_difficulty()
                    } else {
                        sudoku.solve_difficulty()
                    };
                    let _ = output_handle.write_all(&steps.to_string().as_bytes());
                    let _ = output_handle.write_all(b";");
                    let _ = output_handle.write_all(&sudoku.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }

                Mode::CountSolutions(n) => {
                    let count = sudoku.count_solutions(n.n);
                    let _ = output_handle.write_all(&count.to_string().as_bytes());
                    let _ = output_handle.write_all(b";");
                    let _ = output_handle.write_all(&sudoku.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }

                Mode::Generate(n) => {
                    let _ = output_handle.write_all(
                        &sudoku
                            .generate_from_seed(&mut rng, n.cells_to_remove)
                            .to_bytes(),
                    );
                    let _ = output_handle.write_all(b"\n");
                }
                Mode::Info => {
                    info[sudoku.count_solutions(2)] += 1;
                }
            }

            buffer.clear();
        }
        if let Mode::Info = opts.mode {
            println!(
                "0 Solutions: {}, 1 Solution: {}, 2+ Solutions: {}",
                info[0], info[1], info[2]
            );
        }
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::main()
}
