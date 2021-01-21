use crate::*;

pub const fn cells_in_house(square: usize) -> [u8; 20] {
    let column_start = square % 9;
    let row_start = square - column_start;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    let mut squares_to_change: u128 = 0;
    squares_to_change |= ((1 << 9) - 1) << row_start;
    squares_to_change |= (1
        + (1 << 9)
        + (1 << 18)
        + (1 << 27)
        + (1 << 36)
        + (1 << 45)
        + (1 << 54)
        + (1 << 63)
        + (1 << 72))
        << column_start;
    squares_to_change |= (0b111 + (0b111 << 9) + (0b111 << 18)) << box_start;
    squares_to_change &= !(1 << square);
    let mut squares_to_change_array = [0; 20];

    let mut i = 0;
    while i < 20 {
        squares_to_change_array[i] = get_last_digit!(squares_to_change, u8);
        i += 1;
    }
    squares_to_change_array
}

pub const CELLS_TO_CHANGE: [[u8; 20]; 81] = {
    let mut data = [[0; 20]; 81];
    let mut i = 0;
    while i < 81 {
        data[i] = cells_in_house(i);
        i += 1;
    }
    data
};

/** Max 9 bit number */
pub const SUDOKU_MAX: u16 = (1 << 9) - 1;

pub const SOLVED_SUDOKU: u128 = (1 << 81) - 1;

/*
After solving this many squares, do not use pointing pairs
*/
pub const SCANNING_CUTOFF: u32 = 40;
