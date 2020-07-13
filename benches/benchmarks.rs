#[macro_use]
extern crate criterion;
extern crate rand;
extern crate sudoku;

use rand::prelude::*;

use criterion::Criterion;
use std::io::BufRead;
use std::str::FromStr;

fn bench_solving(sudoku: Option<&String>, solver: Solver, mode: Mode) -> usize {
    let solution_count = match mode {
        Mode::SolveOne => 1,
        Mode::SolveUnique => 2,
    };
    match solver {
        Solver::MSolve => {
            if let Ok(sudoku) = msolve::Sudoku::from_str(sudoku.unwrap()) {
                sudoku.count_solutions(solution_count)
            } else {
                0
            }
        }
        Solver::RustSudoku => {
            if let Ok(sudoku) = sudoku::Sudoku::from_str_line(sudoku.unwrap()) {
                sudoku.count_at_most(solution_count)
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Solver {
    MSolve,
    RustSudoku,
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    SolveOne,
    SolveUnique,
}

fn criterion_benchmark(c: &mut Criterion) {
    let paths = [
        ("top2365", true),            // http://magictour.free.fr/top2365
        ("sudoku17", true),           // https://staffhome.ecm.uwa.edu.au/~00013890/sudoku17
        ("kaggle.txt", true),         // https://www.kaggle.com/bryanpark/sudoku
        ("gen_puzzles", true),        // http://www.enjoysudoku.com/gen_puzzles.zip
        ("forum_hardest_1905", true), // http://forum.enjoysudoku.com/the-hardest-sudokus-new-thread-t6539-600.html#p277835
        ("hardest_to_solve", false), // Top 1000 hardest to solve for msolve from forum_hardest_1905
        ("hardest_to_verify", false), // Top 1000 hardest to solve unique for msolve from forum_hardest_1905
        ("most_difficult", false), // Top 100 hardest sudokus to verify across all lists and generated ones
        ("serg_benchmark", true),  // http://sites.google.com/site/sergsudoku/benchmark.zip
    ];

    for (path, shuffle) in paths.iter() {
        let file_in =
            std::fs::File::open(format!("bench_sudokus/{}", path)).expect("Failed to open file");
        let mut buf = std::io::BufReader::new(file_in);

        let mut sudokus = Vec::<String>::new();
        let mut line = String::with_capacity(81);
        while buf.read_line(&mut line).unwrap() > 0 {
            if let Ok(sudoku) = sudoku::Sudoku::from_str_line(&line) {
                sudokus.push((&sudoku.to_str_line()).to_string());
            }
            line.clear();
        }
        if *shuffle {
            for sudoku_string in sudokus.iter_mut() {
                let mut sudoku = sudoku::Sudoku::from_str_line(&*sudoku_string).unwrap();
                sudoku.shuffle();
                *sudoku_string = sudoku.to_string()
            }
            while sudokus.len() < 50000 {
                let len = sudokus.len();
                for i in 0..len {
                    sudokus.push(sudokus[i].clone());
                    sudokus[i] = {
                        let mut sudoku = sudoku::Sudoku::from_str_line(&sudokus[i]).unwrap();
                        sudoku.shuffle();
                        sudoku.to_string()
                    };
                }
            }
        }
        sudokus.shuffle(&mut rand::thread_rng());
        let mut sudoku_iter = sudokus.iter().cycle();
        for mode in &[Mode::SolveOne, Mode::SolveUnique] {
            for solver in &[Solver::MSolve, Solver::RustSudoku] {
                c.bench_function(&format!("{}_{:?}_{:?}", path, solver, mode), |b| {
                    b.iter(|| {
                        criterion::black_box(bench_solving(sudoku_iter.next(), *solver, *mode));
                    })
                });
            }
        }
    }
    let worlds_hardest_sudoku: [u8; 81] = criterion::black_box([
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 7, 0, 0, 9, 0, 2, 0, 0, 0, 5, 0,
        0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 4, 5, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0,
        0, 6, 8, 0, 0, 8, 5, 0, 0, 0, 1, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0,
    ]);
    let hardbrute_sudoku: [u8; 81] = criterion::black_box([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 9,
    ]);
    let easy_8802: [u8; 81] = criterion::black_box([
        0, 5, 0, 4, 0, 0, 9, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 5, 9, 0, 0, 0, 0, 7, 6, 3, 0, 0, 7, 5,
        0, 0, 0, 0, 0, 4, 4, 1, 0, 0, 0, 0, 7, 9, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 9, 0, 0,
        2, 7, 1, 7, 0, 0, 0, 0, 5, 4, 0, 6, 0, 0, 2, 0, 0, 0, 0, 0, 0,
    ]);
    let random17_sudoku: [u8; 81] = criterion::black_box([
        0, 0, 0, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 6, 0, 0, 0, 5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 8, 0, 0, 0, 0, 8, 1,
        0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0,
    ]);
    let solved_sudoku: [u8; 81] = criterion::black_box([
        8, 1, 2, 7, 5, 3, 6, 4, 9, 9, 4, 3, 6, 8, 2, 1, 7, 5, 6, 7, 5, 4, 9, 1, 2, 8, 3, 1, 5, 4,
        2, 3, 7, 8, 9, 6, 3, 6, 9, 8, 4, 5, 7, 2, 1, 2, 8, 7, 1, 6, 9, 5, 3, 4, 5, 2, 1, 9, 7, 4,
        3, 6, 8, 4, 3, 8, 5, 2, 6, 9, 1, 7, 7, 9, 6, 3, 1, 8, 4, 5, 2,
    ]);
    c.bench_function("easy_8802", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&easy_8802).solve_one());
        })
    });
    c.bench_function("World's Hardest Sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&worlds_hardest_sudoku).solve_one());
        })
    });
    c.bench_function("hardbrute_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&hardbrute_sudoku).solve_one());
        })
    });
    c.bench_function("random17_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&random17_sudoku).solve_one());
        })
    });
    c.bench_function("solved_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&solved_sudoku).solve_one());
        })
    });
    c.bench_function("empty_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::empty().solve_one());
        })
    });
    c.bench_function("first 1000 solutions to empty_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::empty().count_solutions(1000));
        })
    });
    #[cfg(feature = "generate")]
    for count_steps in [true, false].iter() {
        let string = if *count_steps {
            "Counting Steps"
        } else {
            "Without Counting Steps"
        };
        c.bench_function(&format!("Generate first {}", string), |b| {
            b.iter(|| {
                criterion::black_box(
                    &msolve::Sudoku::generate(rand::thread_rng(), *count_steps)
                        .next()
                        .unwrap(),
                );
            })
        });
        let mut generator = msolve::Sudoku::generate(rand::thread_rng(), *count_steps);
        c.bench_function(&format!("Generate puzzle {}", string), |b| {
            b.iter(|| {
                criterion::black_box(&generator.next().unwrap());
            })
        });
    }
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
