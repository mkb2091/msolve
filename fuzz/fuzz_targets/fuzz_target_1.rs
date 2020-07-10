#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: [[u8; 27]; 3]| {
    let data = unsafe { std::mem::transmute::<[[u8; 27]; 3], [u8; 81]>(data) };
    let sudoku = msolve::Sudoku::from(data);
    for solution in sudoku.iter().take(2) {
        assert!(solution.to_array().iter().all(|x| *x <= 9 && *x != 0));
        assert!(solution.to_bytes().iter().all(|x| *x != b'.'));
    }
});
