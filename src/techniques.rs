use crate::consts;

pub fn apply_number(sudoku: &mut [u16; 81], square: usize) {
    assert!(square < 81);
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let not_value = consts::SUDOKU_MAX + consts::SQUARE_DONE - value;
    let row_start = square / 9 * 9;
    unsafe {
        *sudoku.get_unchecked_mut(row_start + 8) = (sudoku.get_unchecked(row_start + 8)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 8) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 7) = (sudoku.get_unchecked(row_start + 7)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 7) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 6) = (sudoku.get_unchecked(row_start + 6)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 6) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 5) = (sudoku.get_unchecked(row_start + 5)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 5) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 4) = (sudoku.get_unchecked(row_start + 4)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 4) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 3) = (sudoku.get_unchecked(row_start + 3)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 3) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 2) = (sudoku.get_unchecked(row_start + 2)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 2) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start + 1) = (sudoku.get_unchecked(row_start + 1)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start + 1) & value == value) as u16));
        *sudoku.get_unchecked_mut(row_start) = (sudoku.get_unchecked(row_start) & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(row_start) & value == value) as u16));

        let column_start = square % 9;
        *sudoku.get_unchecked_mut(column_start + 72) = (sudoku.get_unchecked(column_start + 72)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 72) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 63) = (sudoku.get_unchecked(column_start + 63)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 63) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 54) = (sudoku.get_unchecked(column_start + 54)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 54) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 45) = (sudoku.get_unchecked(column_start + 45)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 45) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 36) = (sudoku.get_unchecked(column_start + 36)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 36) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 27) = (sudoku.get_unchecked(column_start + 27)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 27) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 18) = (sudoku.get_unchecked(column_start + 18)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 18) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start + 9) = (sudoku.get_unchecked(column_start + 9)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start + 9) & value == value) as u16));
        *sudoku.get_unchecked_mut(column_start) = (sudoku.get_unchecked(column_start) & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(column_start) & value == value) as u16));

        let box_start = square / 3 % 3 * 3 + square / 27 * 27;
        *sudoku.get_unchecked_mut(box_start + 20) = (sudoku.get_unchecked(box_start + 20)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 20) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 19) = (sudoku.get_unchecked(box_start + 19)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 19) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 18) = (sudoku.get_unchecked(box_start + 18)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 18) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 11) = (sudoku.get_unchecked(box_start + 11)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 11) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 10) = (sudoku.get_unchecked(box_start + 10)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 10) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 9) = (sudoku.get_unchecked(box_start + 9)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 9) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 2) = (sudoku.get_unchecked(box_start + 2)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 2) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start + 1) = (sudoku.get_unchecked(box_start + 1)
            & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start + 1) & value == value) as u16));
        *sudoku.get_unchecked_mut(box_start) = (sudoku.get_unchecked(box_start) & not_value)
            | (consts::SUDOKU_TECHNIQUES_TOTAL
                * ((sudoku.get_unchecked(box_start) & value == value) as u16));

        *sudoku.get_unchecked_mut(square) = value | consts::SQUARE_DONE;
    }
}

pub fn naked_pair(sudoku: &mut [u16; 81], square: usize) {
    assert!(square < 81);
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
    let square = square as u8;
    let not_value = consts::SUDOKU_MAX + consts::SQUARE_DONE - value;
    for house in &[rows, columns, boxes] {
        unsafe {
            if let Some(second) = house.iter().find(|&second| {
                sudoku.get_unchecked(*second as usize) & consts::SUDOKU_VALUES_TOTAL == value
            }) {
                for pos in house.iter() {
                    if *pos != square
                        && pos != second
                        && sudoku.get_unchecked(*pos as usize) & value != 0
                    {
                        *sudoku.get_unchecked_mut(*pos as usize) &= not_value;
                        *sudoku.get_unchecked_mut(*pos as usize) |= consts::SUDOKU_TECHNIQUES_TOTAL;
                    }
                }
                sudoku[*second as usize] &= consts::SUDOKU_MAX - 512;
            }
        }
    }
    sudoku[square as usize] &= consts::SUDOKU_MAX - 512;
}

pub fn naked_triple(sudoku: &mut [u16; 81], square: usize) {
    assert!(square < 81);
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
    let square = square as u8;
    let not_value = consts::SUDOKU_MAX + consts::SQUARE_DONE - value;
    for house in [rows, columns, boxes].iter() {
        unsafe {
            if let Some((i1, pos)) = house[..7].iter().enumerate().find(|(_, &pos)| {
                sudoku.get_unchecked(pos as usize) & consts::SUDOKU_VALUES_TOTAL == value
            }) {
                if let Some(pos2) = house[i1 + 1..].iter().find(|&pos2| {
                    sudoku.get_unchecked(*pos2 as usize) & consts::SUDOKU_VALUES_TOTAL == value
                }) {
                    for pos3 in house.iter() {
                        if pos3 != pos
                            && pos3 != pos2
                            && (sudoku.get_unchecked(*pos3 as usize) & value != 0)
                        {
                            *sudoku.get_unchecked_mut(*pos3 as usize) &= not_value;
                            *sudoku.get_unchecked_mut(*pos3 as usize) |=
                                consts::SUDOKU_TECHNIQUES_TOTAL;
                        }
                    }
                    sudoku[*pos2 as usize] &= consts::SUDOKU_MAX - 512;
                }
                sudoku[*pos as usize] &= consts::SUDOKU_MAX - 512;
            }
        }
    }
    sudoku[square as usize] &= consts::SUDOKU_MAX - 512;
}

#[inline(never)]
pub fn hidden_singles(sudoku: &mut [u16; 81], square: usize) -> bool {
    assert!(square < 81);
    let value = sudoku[square];
    sudoku[square] = 0;
    let row_start = square / 9 * 9;
    let column_start = square % 9;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    let needed = unsafe {
        consts::SUDOKU_VALUES_TOTAL
            - ((sudoku.get_unchecked(row_start + 8)
                | sudoku.get_unchecked(row_start + 7)
                | sudoku.get_unchecked(row_start + 6)
                | sudoku.get_unchecked(row_start + 5)
                | sudoku.get_unchecked(row_start + 4)
                | sudoku.get_unchecked(row_start + 3)
                | sudoku.get_unchecked(row_start + 2)
                | sudoku.get_unchecked(row_start + 1)
                | sudoku.get_unchecked(row_start))
                & (sudoku.get_unchecked(column_start + 72)
                    | sudoku.get_unchecked(column_start + 63)
                    | sudoku.get_unchecked(column_start + 54)
                    | sudoku.get_unchecked(column_start + 45)
                    | sudoku.get_unchecked(column_start + 36)
                    | sudoku.get_unchecked(column_start + 27)
                    | sudoku.get_unchecked(column_start + 18)
                    | sudoku.get_unchecked(column_start + 9)
                    | sudoku.get_unchecked(column_start))
                & (sudoku.get_unchecked(box_start + 20)
                    | sudoku.get_unchecked(box_start + 19)
                    | sudoku.get_unchecked(box_start + 18)
                    | sudoku.get_unchecked(box_start + 11)
                    | sudoku.get_unchecked(box_start + 10)
                    | sudoku.get_unchecked(box_start + 9)
                    | sudoku.get_unchecked(box_start + 2)
                    | sudoku.get_unchecked(box_start + 1)
                    | sudoku.get_unchecked(box_start))
                & consts::SUDOKU_VALUES_TOTAL)
    };
    match consts::OPTION_COUNT_CACHE[needed as usize] {
        0 => {
            sudoku[square] = value & (consts::SUDOKU_MAX - 1024);
            true
        }
        1 => {
            if value & needed != 0 {
                sudoku[square] = (value & needed) | (consts::SUDOKU_TECHNIQUES_TOTAL - 1024);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
