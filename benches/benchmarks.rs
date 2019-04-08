#[macro_use]
extern crate criterion;

use criterion::Criterion;

use msolve;

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
    let mut solver = msolve::MSolve::new();
    c.bench_function("easy_8802", move |b| {
        b.iter(|| {
            solver.set_sudoku(easy_8802);
            solver.apply_techniques();
            criterion::black_box(solver.to_array())
        })
    });
    c.bench_function("World's Hardest Sudoku", move |b| {
        b.iter(|| {
            solver.set_sudoku(worlds_hardest_sudoku);
            solver.apply_techniques();
            criterion::black_box(solver.to_array())
        })
    });
    c.bench_function("hardbrute_sudoku", move |b| {
        b.iter(|| {
            solver.set_sudoku(hardbrute_sudoku);
            solver.apply_techniques();
            criterion::black_box(solver.to_array())
        })
    });
    c.bench_function("empty_sudoku", move |b| {
        b.iter(|| {
            solver.set_sudoku(empty_sudoku);
            solver.apply_techniques();
            criterion::black_box(solver.to_array())
        })
    });
    c.bench_function("random17_sudoku", move |b| {
        b.iter(|| {
            solver.set_sudoku(random17_sudoku);
            solver.apply_techniques();
            criterion::black_box(solver.to_array())
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
