#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
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
    let sudoku_list: [[u8; 81]; 5] = [
        worlds_hardest_sudoku,
        hardbrute_sudoku,
        easy_8802,
        empty_sudoku,
        random17_sudoku,
    ];
    c.bench_function("easy_8802", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(&easy_8802));
        })
    });
    c.bench_function("World's Hardest Sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(&worlds_hardest_sudoku));
        })
    });
    c.bench_function("hardbrute_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(&hardbrute_sudoku));
        })
    });
    c.bench_function("empty_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(&empty_sudoku));
        })
    });
    c.bench_function("random17_sudoku", move |b| {
        b.iter(|| {
            criterion::black_box(&msolve::solve(&random17_sudoku));
        })
    });
    c.bench_function("microbench_from_array", move |b| {
        b.iter(|| {
            for sudoku in &sudoku_list {
                let sudoku = msolve::structures::Sudoku::from_array(&sudoku);
                criterion::black_box(&sudoku);
            }
        })
    });
    c.bench_function("microbench_from_array_and_to_array", move |b| {
        b.iter(|| {
            for sudoku in &sudoku_list {
                let sudoku = msolve::structures::Sudoku::from_array(&sudoku);
                criterion::black_box(&sudoku.to_array());
            }
        })
    });
    c.bench_function("microbench_hidden_singles", move |b| {
        b.iter(|| {
            for sudoku in &sudoku_list {
                let mut sudoku = msolve::structures::Sudoku::from_array(&sudoku);
                for i in 0..81 {
                    msolve::techniques::hidden_singles(&mut sudoku, i);
                }
                criterion::black_box(&sudoku);
            }
        })
    });
    c.bench_function("microbench_apply_number", move |b| {
        b.iter(|| {
            for sudoku in &sudoku_list {
                let mut sudoku = msolve::structures::Sudoku::from_array(&sudoku);
                for i in 0..81 {
                    msolve::techniques::apply_number(&mut sudoku, i);
                }
                criterion::black_box(&sudoku);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
