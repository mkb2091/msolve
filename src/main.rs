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
        #[clap(short, long)]
        verify_uniqueness: bool,
        #[clap(short, long, default_value = "1")]
        step_weight: usize,
        #[clap(short, long, default_value = "1")]
        clue_weight: usize,
        #[clap(subcommand)]
        mode: Mode,
    }

    #[derive(Clap, Copy, Clone)]
    enum Mode {
        Solve(Solve),
        Select(Select),
        Difficulty,
        CountSolutions(CountSolutions),
        Generate(Generate),
        Info,
    }

    #[derive(Clap, Copy, Clone)]
    struct Solve {
        #[clap(short, long, default_value = "1")]
        count: usize,
    }
    #[derive(Clap, Copy, Clone)]
    struct Select {
        #[clap(short, long)]
        invert: bool,
    }

    #[derive(Clap, Copy, Clone)]
    struct CountSolutions {
        n: usize,
    }

    #[derive(Clap, Copy, Clone)]
    struct Generate {
        cells_to_remove: usize,
        #[clap(subcommand)]
        mode: GenerateMode,
        #[clap(short, long)]
        display_score: bool,
    }

    #[derive(Clap, Copy, Clone)]
    enum GenerateMode {
        Once,
        Continuous(GenerateContinuous),
    }

    #[derive(Clap, Copy, Clone)]
    struct GenerateContinuous {
        #[clap(short, long)]
        pool_size: std::num::NonZeroUsize,
        #[clap(short, long, default_value = "0")]
        iterations: usize,
        #[clap(short, long, default_value = "2")]
        growth_factor: std::num::NonZeroUsize,
    }

    fn score_sudoku(sudoku: &msolve::Sudoku, opts: &Opts) -> Option<i32> {
        sudoku.difficulty(opts.verify_uniqueness, opts.step_weight, opts.clue_weight)
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
        let mut generation_pool = Vec::<(msolve::Sudoku, i32)>::new();
        if let Mode::Generate(generate) = opts.mode {
            if let GenerateMode::Continuous(continuous) = generate.mode {
                generation_pool
                    .reserve(continuous.pool_size.get() * continuous.growth_factor.get() + 1);
                generation_pool.push((msolve::Sudoku::generate(&mut rng), i32::MIN));
            }
        }
        while let Ok(result) = input.read_line(&mut buffer) {
            if result == 0 {
                break;
            }
            let sudoku = buffer.parse::<msolve::Sudoku>().unwrap();
            match opts.mode {
                Mode::Solve(solve) => {
                    if opts.verify_uniqueness {
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
                    let mut does_match = if opts.verify_uniqueness {
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
                Mode::Difficulty => {
                    if let Some(difficulty) = score_sudoku(&sudoku, &opts) {
                        let _ = output_handle.write_all(&difficulty.to_string().as_bytes());
                        let _ = output_handle.write_all(b";");
                        let _ = output_handle.write_all(&sudoku.to_bytes());
                        let _ = output_handle.write_all(b"\n");
                    }
                }

                Mode::CountSolutions(n) => {
                    let count = sudoku.count_solutions(n.n);
                    let _ = output_handle.write_all(&count.to_string().as_bytes());
                    let _ = output_handle.write_all(b";");
                    let _ = output_handle.write_all(&sudoku.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                }

                Mode::Generate(generate) => match generate.mode {
                    GenerateMode::Once => {
                        let sudoku = sudoku.generate_from_seed(&mut rng, generate.cells_to_remove);
                        if generate.display_score {
                            if let Some(score) = score_sudoku(&sudoku, &opts) {
                                let _ = output_handle.write_all(&score.to_string().as_bytes());
                                let _ = output_handle.write_all(b";");
                            } else {
                                debug_assert!(false, "Generated sudokus should be valid");
                            }
                        }

                        let _ = output_handle.write_all(&sudoku.to_bytes());
                        let _ = output_handle.write_all(b"\n");
                    }
                    GenerateMode::Continuous(_) => generation_pool.push((sudoku, i32::MIN)),
                },
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

        if let Mode::Generate(generate) = opts.mode {
            if let GenerateMode::Continuous(continuous) = generate.mode {
                let mut pool_2 = Vec::<(msolve::Sudoku, i32)>::with_capacity(
                    continuous.pool_size.get() * continuous.growth_factor.get() + 1,
                );
                let mut iteration = 1;
                while iteration != continuous.iterations {
                    iteration += 1;
                    for (old_sudoku, _) in generation_pool.iter() {
                        for _ in 0..continuous.growth_factor.get() {
                            let sudoku =
                                old_sudoku.generate_from_seed(&mut rng, generate.cells_to_remove);
                            if let Some(score) = score_sudoku(&sudoku, &opts) {
                                // Reinitializing as sudoku contains extra information that makes solving quicker
                                pool_2.push((sudoku, score));
                            } else {
                                debug_assert!(false, "Generated sudokus should be valid");
                            }
                        }
                    }
                    let sudoku = msolve::Sudoku::generate(&mut rng);
                    if let Some(score) = score_sudoku(&sudoku, &opts) {
                        pool_2.push((sudoku, score));
                    } else {
                        debug_assert!(false, "Generated sudokus should be valid");
                    }

                    pool_2.sort_unstable_by(|a, b| b.1.cmp(&a.1));
                    pool_2.dedup();

                    for (sudoku, score) in pool_2.iter().rev() {
                        if generate.display_score {
                            let _ = output_handle.write_all(&score.to_string().as_bytes());
                            let _ = output_handle.write_all(b";");
                        }
                        let _ = output_handle.write_all(&sudoku.to_bytes());
                        let _ = output_handle.write_all(b"\n");
                    }

                    pool_2.shrink_to_fit(); // Incase large input

                    pool_2.truncate(continuous.pool_size.get());

                    std::mem::swap(&mut generation_pool, &mut pool_2);
                    pool_2.clear();
                }
            }
        }
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::main()
}
