#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::convert::TryFrom;

#[derive(Arbitrary, Debug)]
struct Sudoku {
    start: [u8; 27],
    middle: [u8; 27],
    end: [u8; 27],
}

fuzz_target!(|data: Sudoku| {
    let (start, middle, end) = (data.start, data.middle, data.end);
    let data = start
        .iter()
        .chain(middle.iter())
        .chain(end.iter())
        .map(|x| *x)
        .collect::<Vec<u8>>();
    if let Ok(sudoku) = msolve::Sudoku::try_from(data) {
        for solution in sudoku.iter().take(2) {
            assert!(solution.to_array().iter().all(|x| *x <= 9 && *x != 0));
            assert!(solution.to_bytes().iter().all(|x| *x != b'.'));
        }
    }
});
