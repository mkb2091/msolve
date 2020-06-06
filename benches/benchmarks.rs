#[macro_use]
extern crate criterion;
extern crate sudoku;

use criterion::Criterion;
use std::io::BufRead;

fn criterion_benchmark(c: &mut Criterion) {
    // top2365 is from http://magictour.free.fr/top2365
    let file_in = std::fs::File::open("bench_sudokus/top2365").expect("Failed to open file");
    let mut buf = std::io::BufReader::new(file_in);
    let mut top2365_msolve = Vec::<String>::new();
    let mut top2365_sudoku = Vec::<String>::new();
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if let Ok(sudoku) = sudoku::Sudoku::from_str_line(&line) {
            top2365_msolve.push(line.clone());
            top2365_sudoku.push(line.clone());
        }
        line.clear();
    }
    // sudoku17 is from https://staffhome.ecm.uwa.edu.au/~00013890/sudoku17
    let file_in = std::fs::File::open("bench_sudokus/sudoku17").expect("Failed to open file");
    let mut buf = std::io::BufReader::new(file_in);
    let mut sudoku17_msolve = Vec::<String>::new();
    let mut sudoku17_sudoku = Vec::<String>::new();
    let mut line = String::with_capacity(81);
    while buf.read_line(&mut line).unwrap() > 0 {
        if let Ok(sudoku) = sudoku::Sudoku::from_str_line(&line) {
            sudoku17_msolve.push(line.clone());
            sudoku17_sudoku.push(line.clone());
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
    let solver = msolve::Solver::new();
    c.bench_function("top2365_msolve", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve(msolve::str_to_sudoku(&top2365_msolve[i])));
            i += 1;
            i %= top2365_msolve.len();
        })
    });
    c.bench_function("top2365_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(&top2365_sudoku[i])
                    .unwrap()
                    .solve_unique(),
            );
            i += 1;
            i %= top2365_sudoku.len();
        })
    });
    c.bench_function("sudoku17_msolve", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve(msolve::str_to_sudoku(&sudoku17_msolve[i])));
            i += 1;
            i %= sudoku17_msolve.len();
        })
    });
    c.bench_function("sudoku17_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(
                &sudoku::Sudoku::from_str_line(&sudoku17_sudoku[i])
                    .unwrap()
                    .solve_unique(),
            );
            i += 1;
            i %= sudoku17_sudoku.len();
        })
    });
    c.bench_function("easy_8802", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve_array(&easy_8802));
        })
    });
    c.bench_function("World's Hardest Sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve_array(&worlds_hardest_sudoku));
        })
    });
    c.bench_function("hardbrute_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve_array(&hardbrute_sudoku));
        })
    });
    c.bench_function("empty_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve_array(&empty_sudoku));
        })
    });
    c.bench_function("random17_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&solver.solve_array(&random17_sudoku));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
