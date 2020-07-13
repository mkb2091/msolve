#[cfg(feature = "cli")]
mod cli {
    #[cfg(not(feature = "std"))]
    compile_error!("`std` feature is required for cli");

    use std::io::BufRead;
    use std::io::Write;

    use clap::Clap;

    #[derive(Clap, Copy, Clone)]
    #[clap(version = "1.0")]
    struct Opts {
        #[clap(short, long)]
        verify_uniqueness: bool,
        #[clap(short, long)]
        count_steps: bool,
        #[clap(subcommand)]
        mode: Mode,
    }

    #[derive(Clap, Copy, Clone)]
    enum Mode {
        Solve(Solve),
        Select(Select),
        Difficulty,
        CountSolutions(CountSolutions),
        #[cfg(feature = "generate")]
        Generate(Generate),
        ListTechniques,
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

    #[cfg(feature = "generate")]
    #[derive(Clap, Copy, Clone)]
    struct Generate {
        #[clap(subcommand)]
        mode: GenerateMode,
        #[clap(short, long)]
        display_score: bool,
    }

    #[cfg(feature = "generate")]
    #[derive(Clap, Copy, Clone)]
    enum GenerateMode {
        Once(GenerateOnce),
        Continuous(GenerateContinuous),
    }

    #[cfg(feature = "generate")]
    #[derive(Clap, Copy, Clone)]
    struct GenerateOnce {
        cells_to_remove: usize,
    }

    #[cfg(feature = "generate")]
    #[derive(Clap, Copy, Clone)]
    struct GenerateContinuous {
        #[clap(short, long)]
        n: Option<std::num::NonZeroUsize>,
    }

    fn score_sudoku(sudoku: &msolve::Sudoku, opts: &Opts) -> Option<i32> {
        sudoku.difficulty(opts.count_steps)
    }

    pub fn main() {
        let opts: Opts = Opts::parse();

        let stdin = std::io::stdin();
        let mut input = stdin.lock();
        let mut buffer = String::with_capacity(82);
        let stdout = std::io::stdout();
        let mut output_handle = stdout.lock();
        let mut info = [0; 3];
        #[cfg(feature = "rand")]
        let mut rng = rand::thread_rng();
        #[cfg(feature = "generate")]
        if let Mode::Generate(generate) = opts.mode {
            if let GenerateMode::Continuous(continuous) = generate.mode {
                let n = continuous.n.map(|n| n.get()).unwrap_or(0);
                let mut counter = 0;
                for (sudoku, score) in
                    msolve::Sudoku::generate(rand::thread_rng(), opts.count_steps)
                {
                    if generate.display_score {
                        let _ = output_handle.write_all(&score.to_string().as_bytes());
                        let _ = output_handle.write_all(b";");
                    }
                    let _ = output_handle.write_all(&sudoku.to_bytes());
                    let _ = output_handle.write_all(b"\n");
                    if n != 0 {
                        counter += 1;
                        if counter >= n {
                            return;
                        }
                    }
                }
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
                #[cfg(feature = "generate")]
                Mode::Generate(generate) => {
                    if let GenerateMode::Once(once) = generate.mode {
                        let (sudoku, score) = sudoku.generate_from_seed(
                            &mut rng,
                            once.cells_to_remove,
                            opts.count_steps,
                        );
                        if generate.display_score {
                            let _ = output_handle.write_all(&score.to_string().as_bytes());
                            let _ = output_handle.write_all(b";");
                        }

                        let _ = output_handle.write_all(&sudoku.to_bytes());
                        let _ = output_handle.write_all(b"\n");
                    } else {
                        unimplemented!()
                    }
                }
                Mode::ListTechniques => {
                    for (explanation, state) in sudoku.list_techniques().iter() {
                        let _ = output_handle.write_all(&explanation.as_bytes());
                        let _ = output_handle.write_all(b"\n");
                        let _ = output_handle.write_all(&state.to_pencilmark_bytes());
                        let _ = output_handle.write_all(b"\n");
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
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
