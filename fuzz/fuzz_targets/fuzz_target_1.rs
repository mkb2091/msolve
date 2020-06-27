#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let sudoku = msolve::Sudoku::from(data);
    assert_eq!(
        sudoku.has_single_solution(),
        sudoku.solve_unique().is_some()
    );
    if sudoku.solve().is_some() {
        if sudoku.has_single_solution() {
            assert!(sudoku.solve_unique_difficulty() >= sudoku.solve_difficulty())
        } else {
            assert!(sudoku.solve_unique_difficulty() > sudoku.solve_difficulty())
        }
    } else {
        assert!(sudoku.solve_unique_difficulty() == sudoku.solve_difficulty())
    }
});
