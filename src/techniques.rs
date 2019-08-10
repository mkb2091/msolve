use crate::consts;
use crate::structures;

#[inline(never)]
pub fn apply_number(sudoku: &mut structures::Sudoku, square: usize) {
    let value = sudoku.options[square];
    let not_value = consts::SUDOKU_MAX - value;
    let column_start = square % 9;
    let row_start = square - column_start;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    sudoku.options[row_start + 8] &= not_value;
    sudoku.options[row_start + 7] &= not_value;
    sudoku.options[row_start + 6] &= not_value;
    sudoku.options[row_start + 5] &= not_value;
    sudoku.options[row_start + 4] &= not_value;
    sudoku.options[row_start + 3] &= not_value;
    sudoku.options[row_start + 2] &= not_value;
    sudoku.options[row_start + 1] &= not_value;
    sudoku.options[row_start] &= not_value;

    sudoku.options[column_start + 72] &= not_value;
    sudoku.options[column_start + 63] &= not_value;
    sudoku.options[column_start + 54] &= not_value;
    sudoku.options[column_start + 45] &= not_value;
    sudoku.options[column_start + 36] &= not_value;
    sudoku.options[column_start + 27] &= not_value;
    sudoku.options[column_start + 18] &= not_value;
    sudoku.options[column_start + 9] &= not_value;
    sudoku.options[column_start] &= not_value;

    sudoku.options[box_start + 20] &= not_value;
    sudoku.options[box_start + 19] &= not_value;
    sudoku.options[box_start + 18] &= not_value;
    sudoku.options[box_start + 11] &= not_value;
    sudoku.options[box_start + 10] &= not_value;
    sudoku.options[box_start + 9] &= not_value;
    sudoku.options[box_start + 2] &= not_value;
    sudoku.options[box_start + 1] &= not_value;
    sudoku.options[box_start] &= not_value;
    sudoku.options[square] = value;
}

#[inline(never)]
pub fn hidden_singles(sudoku: &mut structures::Sudoku, square: usize) -> bool {
    let value = sudoku.options[square];
    sudoku.options[square] = 0;
    let row_start = square / 9 * 9;
    let column_start = square % 9;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    let needed = consts::SUDOKU_MAX
        - ((sudoku.options[row_start + 8]
            | sudoku.options[row_start + 7]
            | sudoku.options[row_start + 6]
            | sudoku.options[row_start + 5]
            | sudoku.options[row_start + 4]
            | sudoku.options[row_start + 3]
            | sudoku.options[row_start + 2]
            | sudoku.options[row_start + 1]
            | sudoku.options[row_start])
            & (sudoku.options[column_start + 72]
                | sudoku.options[column_start + 63]
                | sudoku.options[column_start + 54]
                | sudoku.options[column_start + 45]
                | sudoku.options[column_start + 36]
                | sudoku.options[column_start + 27]
                | sudoku.options[column_start + 18]
                | sudoku.options[column_start + 9]
                | sudoku.options[column_start])
            & (sudoku.options[box_start + 20]
                | sudoku.options[box_start + 19]
                | sudoku.options[box_start + 18]
                | sudoku.options[box_start + 11]
                | sudoku.options[box_start + 10]
                | sudoku.options[box_start + 9]
                | sudoku.options[box_start + 2]
                | sudoku.options[box_start + 1]
                | sudoku.options[box_start]));
    match consts::OPTION_COUNT_CACHE[needed as usize] {
        0 => {
            sudoku.options[square] = value;
            true
        }
        1 => {
            if value & needed != 0 {
                sudoku.options[square] = value & needed;
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
