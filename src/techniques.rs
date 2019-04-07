use crate::consts;

pub fn apply_number(sudoku: &mut [u16; 81], square: usize) {
    assert!(square < 81);
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let not_value = consts::SUDOKU_MAX + consts::SQUARE_DONE - value;
    let row_start = square / 9 * 9;
    sudoku[row_start + 8] = (sudoku[row_start + 8] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 8] & value == value) as u16));
    sudoku[row_start + 7] = (sudoku[row_start + 7] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 7] & value == value) as u16));
    sudoku[row_start + 6] = (sudoku[row_start + 6] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 6] & value == value) as u16));
    sudoku[row_start + 5] = (sudoku[row_start + 5] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 5] & value == value) as u16));
    sudoku[row_start + 4] = (sudoku[row_start + 4] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 4] & value == value) as u16));
    sudoku[row_start + 3] = (sudoku[row_start + 3] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 3] & value == value) as u16));
    sudoku[row_start + 2] = (sudoku[row_start + 2] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 2] & value == value) as u16));
    sudoku[row_start + 1] = (sudoku[row_start + 1] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 1] & value == value) as u16));
    sudoku[row_start] = (sudoku[row_start] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start] & value == value) as u16));

    let column_start = square % 9;
    sudoku[column_start + 72] = (sudoku[column_start + 72] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 72] & value == value) as u16));
    sudoku[column_start + 63] = (sudoku[column_start + 63] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 63] & value == value) as u16));
    sudoku[column_start + 54] = (sudoku[column_start + 54] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 54] & value == value) as u16));
    sudoku[column_start + 45] = (sudoku[column_start + 45] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 45] & value == value) as u16));
    sudoku[column_start + 36] = (sudoku[column_start + 36] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 36] & value == value) as u16));
    sudoku[column_start + 27] = (sudoku[column_start + 27] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 27] & value == value) as u16));
    sudoku[column_start + 18] = (sudoku[column_start + 18] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 18] & value == value) as u16));
    sudoku[column_start + 9] = (sudoku[column_start + 9] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 9] & value == value) as u16));
    sudoku[column_start] = (sudoku[column_start] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start] & value == value) as u16));

    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    sudoku[box_start + 20] = (sudoku[box_start + 20] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 20] & value == value) as u16));
    sudoku[box_start + 19] = (sudoku[box_start + 19] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 19] & value == value) as u16));
    sudoku[box_start + 18] = (sudoku[box_start + 18] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 18] & value == value) as u16));
    sudoku[box_start + 11] = (sudoku[box_start + 11] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 11] & value == value) as u16));
    sudoku[box_start + 10] = (sudoku[box_start + 10] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 10] & value == value) as u16));
    sudoku[box_start + 9] = (sudoku[box_start + 9] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 9] & value == value) as u16));
    sudoku[box_start + 2] = (sudoku[box_start + 2] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 2] & value == value) as u16));
    sudoku[box_start + 1] = (sudoku[box_start + 1] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start + 1] & value == value) as u16));
    sudoku[box_start] = (sudoku[box_start] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[box_start] & value == value) as u16));

    sudoku[square] = value | consts::SQUARE_DONE;
}

pub fn naked_pair(sudoku: &mut [u16; 81], square: usize) {
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
    let square = square as u8;
    let not_value = consts::SUDOKU_MAX + consts::SQUARE_DONE - value;
    for (is_box, house) in &[(false, rows), (false, columns), (true, boxes)] {
        if let Some(second) = house
            .iter()
            .find(|&second| sudoku[*second as usize] & consts::SUDOKU_VALUES_TOTAL == value)
        {
            for pos in house.iter() {
                if *pos != square && pos != second && sudoku[*pos as usize] & value != 0 {
                    sudoku[*pos as usize] &= not_value;
                    sudoku[*pos as usize] |= consts::SUDOKU_TECHNIQUES_TOTAL;
                }
            }
            if !is_box {
                if boxes.contains(&second) {
                    for pos in boxes.iter() {
                        if *pos != square && pos != second && sudoku[*pos as usize] & value != 0 {
                            sudoku[*pos as usize] &= not_value;
                            sudoku[*pos as usize] |= consts::SUDOKU_TECHNIQUES_TOTAL;
                        }
                    }
                }
            } else if rows.contains(&second) {
                for pos in rows.iter() {
                    if *pos != square && pos != second && sudoku[*pos as usize] & value != 0 {
                        sudoku[*pos as usize] &= not_value;
                        sudoku[*pos as usize] |= consts::SUDOKU_TECHNIQUES_TOTAL;
                    }
                }
            } else if columns.contains(&second) {
                for pos in columns.iter() {
                    if *pos != square && pos != second && sudoku[*pos as usize] & value != 0 {
                        sudoku[*pos as usize] &= not_value;
                        sudoku[*pos as usize] |= consts::SUDOKU_TECHNIQUES_TOTAL;
                    }
                }
            }
        }
    }
    sudoku[square as usize] &= consts::SUDOKU_MAX - 512;
}

pub fn naked_triple(sudoku: &mut [u16; 81], square: usize) {
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
    let square = square as u8;
    let not_value = consts::SUDOKU_MAX + consts::SQUARE_DONE - value;
    for house in [rows, columns, boxes].iter() {
        if let Some((i1, pos)) = house[..7]
            .iter()
            .enumerate()
            .find(|(_, &pos)| sudoku[pos as usize] & consts::SUDOKU_VALUES_TOTAL == value)
        {
            if let Some(pos2) = house[i1..].iter().find(|&pos2| {
                pos2 != pos && sudoku[*pos2 as usize] & consts::SUDOKU_VALUES_TOTAL == value
            }) {
                for pos3 in house.iter() {
                    if pos3 != pos && pos3 != pos2 && (sudoku[*pos3 as usize] & value != 0) {
                        sudoku[*pos3 as usize] &= not_value;
                        sudoku[*pos3 as usize] |= consts::SUDOKU_TECHNIQUES_TOTAL;
                    }
                }
            }
        }
    }
    sudoku[square as usize] &= consts::SUDOKU_MAX - 512;
}

#[inline(never)]
pub fn hidden_singles(sudoku: &mut [u16; 81], square: usize) -> bool {
    let value = sudoku[square];
    sudoku[square] = 0;
    let row_start = square / 9 * 9;
    let column_start = square % 9;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    let needed = consts::SUDOKU_VALUES_TOTAL
        - ((sudoku[row_start + 8]
            | sudoku[row_start + 7]
            | sudoku[row_start + 6]
            | sudoku[row_start + 5]
            | sudoku[row_start + 4]
            | sudoku[row_start + 3]
            | sudoku[row_start + 2]
            | sudoku[row_start + 1]
            | sudoku[row_start])
            & (sudoku[column_start + 72]
                | sudoku[column_start + 63]
                | sudoku[column_start + 54]
                | sudoku[column_start + 45]
                | sudoku[column_start + 36]
                | sudoku[column_start + 27]
                | sudoku[column_start + 18]
                | sudoku[column_start + 9]
                | sudoku[column_start])
            & (sudoku[box_start + 20]
                | sudoku[box_start + 19]
                | sudoku[box_start + 18]
                | sudoku[box_start + 11]
                | sudoku[box_start + 10]
                | sudoku[box_start + 9]
                | sudoku[box_start + 2]
                | sudoku[box_start + 1]
                | sudoku[box_start])
            & consts::SUDOKU_VALUES_TOTAL);
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
