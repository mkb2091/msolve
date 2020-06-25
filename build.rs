use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn generate_cells_to_change() -> String {
    let mut data: Vec<Vec<usize>> = Vec::with_capacity(81);

    for square in 0..81 {
        let column_start = square % 9;
        let row_start = square - column_start;
        let box_start = square / 3 % 3 * 3 + square / 27 * 27;
        let mut squares_to_change: u128 = 0;
        for (i, box_offset) in [20, 19, 18, 11, 10, 9, 2, 1, 0].iter().enumerate() {
            squares_to_change |= 1 << (row_start + i);
            squares_to_change |= 1 << (column_start + i * 9);
            squares_to_change |= 1 << (box_start + box_offset);
        }
        squares_to_change &= !(1 << square);
        assert_eq!(squares_to_change.count_ones(), 20);
        let mut squares_to_change_array = [0; 20];
        for i in squares_to_change_array.iter_mut() {
            let s2 = squares_to_change.trailing_zeros();
            squares_to_change -= 1 << s2;
            *i = s2 as usize;
        }
        data.push(squares_to_change_array.to_vec());
    }
    format!("const CELLS_TO_CHANGE: [[u8; 20]; 81] = {:?};\n", data)
}

fn generate_consts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("consts.rs");

    let mut f = File::create(&dest_path).unwrap();
    let _ = f.write_all(generate_cells_to_change().as_bytes());
}

fn main() {
    generate_consts()
}
