#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;


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
        .map(|x|*x)
        .collect::<Vec<u8>>();
    if let Some(solution) = msolve::Sudoku::from(data).solve() {
        assert!(solution.to_array()[0] <= 9);
    }
});
