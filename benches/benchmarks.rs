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

    // kaggle is from https://www.kaggle.com/bryanpark/sudoku
    let file_in = std::fs::File::open("bench_sudokus/kaggle.txt").expect("Failed to open file");
    let mut buf = std::io::BufReader::new(file_in);
    let mut kaggle = Vec::<String>::new();
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if sudoku::Sudoku::from_str_line(&line).is_ok() {
            kaggle.push(line.clone());
        }
        line.clear();
    }

    let mut top2365_iter = top2365.iter().cycle();
    let mut sudoku17_iter = sudoku17.iter().cycle();
    let mut kaggle_iter = sudoku17.iter().cycle();

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

    c.bench_function("top2365_msolve", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(top2365_iter.next().unwrap()).solve());
        })
    });

    c.bench_function("top2365_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(top2365_iter.next().unwrap())
                    .unwrap()
                    .solve_one(),
            );
        })
    });

    c.bench_function("top2365_msolve_unique", |b| {
        b.iter(|| {
            criterion::black_box(
                &msolve::Sudoku::from(top2365_iter.next().unwrap()).solve_unique(),
            );
        })
    });

    c.bench_function("top2365_sudoku_unique", |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(top2365_iter.next().unwrap())
                    .unwrap()
                    .solve_unique(),
            );
        })
    });

    c.bench_function("sudoku17_msolve", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(sudoku17_iter.next().unwrap()).solve());
        })
    });

    c.bench_function("sudoku17_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(sudoku17_iter.next().unwrap())
                    .unwrap()
                    .solve_one(),
            );
        })
    });

    c.bench_function("sudoku17_msolve_unique", |b| {
        b.iter(|| {
            criterion::black_box(
                &msolve::Sudoku::from(sudoku17_iter.next().unwrap()).solve_unique(),
            );
        })
    });

    c.bench_function("sudoku17_sudoku_unique", |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(sudoku17_iter.next().unwrap())
                    .unwrap()
                    .solve_unique(),
            );
        })
    });
    c.bench_function("kaggle_msolve", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(kaggle_iter.next().unwrap()).solve());
        })
    });

    c.bench_function("kaggle_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(kaggle_iter.next().unwrap())
                    .unwrap()
                    .solve_one(),
            );
        })
    });

    c.bench_function("kaggle_msolve_unique", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(kaggle_iter.next().unwrap()).solve_unique());
        })
    });

    c.bench_function("kaggle_sudoku_unique", |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(kaggle_iter.next().unwrap())
                    .unwrap()
                    .solve_unique(),
            );
        })
    });
    c.bench_function("easy_8802", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&easy_8802).solve());
        })
    });
    c.bench_function("World's Hardest Sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&worlds_hardest_sudoku).solve());
        })
    });
    c.bench_function("hardbrute_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&hardbrute_sudoku).solve());
        })
    });
    c.bench_function("random17_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::from(&random17_sudoku).solve());
        })
    });

    c.bench_function("empty_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::empty().solve());
        })
    });
    c.bench_function("first 1000 solutions to empty_sudoku", |b| {
        b.iter(|| {
            criterion::black_box(&msolve::Sudoku::empty().count_solutions(1000));
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
