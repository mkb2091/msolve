#[macro_use]
extern crate criterion;
extern crate sudoku;

use criterion::Criterion;
use std::io::BufRead;

fn criterion_benchmark(c: &mut Criterion) {
    // top2365 is from http://magictour.free.fr/top2365
    let file_in = std::fs::File::open("bench_sudokus/top2365").expect("Failed to open file");
    let mut buf = std::io::BufReader::new(file_in);
    let mut top2365 = Vec::<String>::new();
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if sudoku::Sudoku::from_str_line(&line).is_ok() {
            top2365.push(line.clone());
        }
        line.clear();
    }
    // sudoku17 is from https://staffhome.ecm.uwa.edu.au/~00013890/sudoku17
    let file_in = std::fs::File::open("bench_sudokus/sudoku17").expect("Failed to open file");
    let mut buf = std::io::BufReader::new(file_in);
    let mut sudoku17 = Vec::<String>::new();
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if sudoku::Sudoku::from_str_line(&line).is_ok() {
            sudoku17.push(line.clone());
        }
        line.clear();
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
    let empty_sudoku: [u8; 81] = criterion::black_box([0; 81]);
    let random17_sudoku: [u8; 81] = criterion::black_box([
        0, 0, 0, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 6, 0, 0, 0, 5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 8, 0, 0, 0, 0, 8, 1,
        0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0,
    ]);
    let mut i = 0;
    {
        let top2365_msolve = top2365.clone();
        c.bench_function("top2365_msolve", move |b| {
            b.iter(|| {
                criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(
                    &top2365_msolve[i],
                )));
                i += 1;
                i %= top2365_msolve.len();
            })
        });
    }
    {
        let top2365 = top2365.clone();
        c.bench_function("top2365_sudoku", move |b| {
            b.iter(|| {
                criterion::black_box(
                    &sudoku::Sudoku::from_str_line(&top2365[i])
                        .unwrap()
                        .solve_one(),
                );
                i += 1;
                i %= top2365.len();
            })
        });
    }
    {
        let top2365_msolve = top2365.clone();
        c.bench_function("top2365_msolve_unique", move |b| {
            b.iter(|| {
                criterion::black_box(&msolve::solve_unique(msolve::SudokuStruct::from(
                    &top2365_msolve[i],
                )));
                i += 1;
                i %= top2365_msolve.len();
            })
        });
    }
    c.bench_function("top2365_sudoku_unique", move |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(&top2365[i])
                    .unwrap()
                    .solve_unique(),
            );
            i += 1;
            i %= top2365.len();
        })
    });
    {
        let sudoku17_msolve = sudoku17.clone();
        c.bench_function("sudoku17_msolve", move |b| {
            b.iter(|| {
                criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(
                    &sudoku17_msolve[i],
                )));
                i %= sudoku17_msolve.len();
            })
        });
    }
    {
        let sudoku17 = sudoku17.clone();
        c.bench_function("sudoku17_sudoku", move |b| {
            b.iter(|| {
                criterion::black_box(
                    &sudoku::Sudoku::from_str_line(&sudoku17[i])
                        .unwrap()
                        .solve_one(),
                );
                i += 1;
                i %= sudoku17.len();
            })
        });
    }
    {
        let sudoku17_msolve = sudoku17.clone();
        c.bench_function("sudoku17_msolve_unique", move |b| {
            b.iter(|| {
                criterion::black_box(&msolve::solve_unique(msolve::SudokuStruct::from(
                    &sudoku17_msolve[i],
                )));
                i += 1;
                i %= sudoku17_msolve.len();
            })
        });
    }

    c.bench_function("sudoku17_sudoku_unique", move |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(&sudoku17[i])
                    .unwrap()
                    .solve_unique()
            );
            i += 1;
            i %= sudoku17.len();
        })
    });
    c.bench_function("easy_8802", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(&easy_8802)));
        })
    });
    c.bench_function("World's Hardest Sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(
                &worlds_hardest_sudoku,
            )));
        })
    });
    c.bench_function("hardbrute_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(
                &hardbrute_sudoku,
            )));
        })
    });
    c.bench_function("empty_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(&empty_sudoku)));
        })
    });
    c.bench_function("first 1000 solutions to empty_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::count_solutions(
                msolve::SudokuStruct::from(&empty_sudoku),
                1000,
            ));
        })
    });
    c.bench_function("random17_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(msolve::SudokuStruct::from(&random17_sudoku)));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
