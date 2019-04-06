use crate::consts;

pub fn apply_number(sudoku: &mut [u16; 81], square: usize) {
    assert!(square < 81);
    let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
    let other_techniques = sudoku[square] & (consts::SUDOKU_TECHNIQUES_TOTAL);
    let not_value = consts::SUDOKU_MAX - value;
    let row_start = square / 9 * 9;
    sudoku[row_start + 8] = (sudoku[row_start + 8] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 8] & value) / value));
    sudoku[row_start + 7] = (sudoku[row_start + 7] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 7] & value) / value));
    sudoku[row_start + 6] = (sudoku[row_start + 6] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 6] & value) / value));
    sudoku[row_start + 5] = (sudoku[row_start + 5] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 5] & value) / value));
    sudoku[row_start + 4] = (sudoku[row_start + 4] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 4] & value) / value));
    sudoku[row_start + 3] = (sudoku[row_start + 3] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 3] & value) / value));
    sudoku[row_start + 2] = (sudoku[row_start + 2] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 2] & value) / value));
    sudoku[row_start + 1] = (sudoku[row_start + 1] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start + 1] & value) / value));
    sudoku[row_start] = (sudoku[row_start] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[row_start] & value) / value));

    let column_start = square % 9;
    sudoku[column_start + 72] = (sudoku[column_start + 72] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 72] & value) / value));
    sudoku[column_start + 63] = (sudoku[column_start + 63] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 63] & value) / value));
    sudoku[column_start + 54] = (sudoku[column_start + 54] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 54] & value) / value));
    sudoku[column_start + 45] = (sudoku[column_start + 45] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 45] & value) / value));
    sudoku[column_start + 36] = (sudoku[column_start + 36] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 36] & value) / value));
    sudoku[column_start + 27] = (sudoku[column_start + 27] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 27] & value) / value));
    sudoku[column_start + 18] = (sudoku[column_start + 18] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 18] & value) / value));
    sudoku[column_start + 9] = (sudoku[column_start + 9] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start + 9] & value) / value));
    sudoku[column_start] = (sudoku[column_start] & not_value)
        | (consts::SUDOKU_TECHNIQUES_TOTAL * ((sudoku[column_start] & value) / value));

    let (_, _, boxes) = consts::PRECOMPUTED_INDEXES[square];
    for cbox in boxes.iter() {
        if sudoku[*cbox as usize] & value != 0 {
            sudoku[*cbox as usize] =
                (sudoku[*cbox as usize] ^ (value)) | consts::SUDOKU_TECHNIQUES_TOTAL;
        }
    }
    sudoku[square] = value | other_techniques;
}

#[inline(never)]
pub fn naked_n(sudoku: &mut [u16; 81]) -> bool {
    let mut changed: bool = false;
    for square in 0..81 {
        if sudoku[square] & 512 == 512 {
            let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
            let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
            // Naked singles
            if consts::OPTION_COUNT_CACHE[value as usize] == 1 {
                apply_number(sudoku, square);
                changed = true;
            // Naked Pairs
            } else if consts::OPTION_COUNT_CACHE[value as usize] == 2 {
                for house in [rows, columns, boxes].iter() {
                    for pos in house[..7].iter() {
                        if sudoku[*pos as usize] & consts::SUDOKU_VALUES_TOTAL == value {
                            for pos2 in house.iter() {
                                if pos2 != pos && (sudoku[*pos2 as usize] & value != 0) {
                                    sudoku[*pos2 as usize] = (sudoku[*pos2 as usize] ^ (value))
                                        | consts::SUDOKU_TECHNIQUES_TOTAL;
                                    changed = true;
                                }
                            }
                            break;
                        }
                    }
                }
            // Naked Triples
            // Currently only works for 3 identical cells
            } else if consts::OPTION_COUNT_CACHE[value as usize] == 3 {
                for house in [rows, columns, boxes].iter() {
                    for (i1, pos) in house[..6].iter().enumerate() {
                        if sudoku[*pos as usize] & consts::SUDOKU_VALUES_TOTAL == value {
                            for pos2 in house[i1..7].iter() {
                                if pos2 != pos
                                    && sudoku[*pos2 as usize] & consts::SUDOKU_VALUES_TOTAL == value
                                {
                                    for pos3 in house.iter() {
                                        if pos3 != pos
                                            && pos3 != pos2
                                            && (sudoku[*pos3 as usize] & value != 0)
                                        {
                                            sudoku[*pos3 as usize] = (sudoku[*pos3 as usize]
                                                ^ (value))
                                                | consts::SUDOKU_TECHNIQUES_TOTAL;
                                            changed = true;
                                        }
                                    }
                                    break;
                                }
                            }
                            break;
                        }
                    }
                }
            }
            sudoku[square] -= 512;
        }
    }
    changed
}

pub fn hidden_singles(sudoku: &mut [u16; 81]) -> bool {
    let mut changed: bool = false;
    for square in 0..81 {
        if sudoku[square] & 1024 == 1024 {
            let mut value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
            let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
            let row_total: u16 = consts::SUDOKU_VALUES_TOTAL
                - ((sudoku[rows[7] as usize]
                    | sudoku[rows[6] as usize]
                    | sudoku[rows[5] as usize]
                    | sudoku[rows[4] as usize]
                    | sudoku[rows[3] as usize]
                    | sudoku[rows[2] as usize]
                    | sudoku[rows[1] as usize]
                    | sudoku[rows[0] as usize])
                    & consts::SUDOKU_VALUES_TOTAL);
            match consts::OPTION_COUNT_CACHE[row_total as usize] {
                0 => {}
                1 => {
                    if value & row_total != 0 {
                        value &= row_total;
                        changed = true;
                    } else {
                        sudoku[square] = 0;
                        return false;
                    }
                }
                _ => {
                    sudoku[square] = 0;
                    return false;
                }
            }
            let column_total: u16 = consts::SUDOKU_VALUES_TOTAL
                - ((sudoku[columns[7] as usize]
                    | sudoku[columns[6] as usize]
                    | sudoku[columns[5] as usize]
                    | sudoku[columns[4] as usize]
                    | sudoku[columns[3] as usize]
                    | sudoku[columns[2] as usize]
                    | sudoku[columns[1] as usize]
                    | sudoku[columns[0] as usize])
                    & consts::SUDOKU_VALUES_TOTAL);
            match consts::OPTION_COUNT_CACHE[column_total as usize] {
                0 => {}
                1 => {
                    if value & column_total != 0 {
                        value &= column_total;
                        changed = true;
                    } else {
                        sudoku[square] = 0;
                        return false;
                    }
                }
                _ => {
                    sudoku[square] = 0;
                    return false;
                }
            }
            let box_total: u16 = consts::SUDOKU_VALUES_TOTAL
                - ((sudoku[boxes[7] as usize]
                    | sudoku[boxes[6] as usize]
                    | sudoku[boxes[5] as usize]
                    | sudoku[boxes[4] as usize]
                    | sudoku[boxes[3] as usize]
                    | sudoku[boxes[2] as usize]
                    | sudoku[boxes[1] as usize]
                    | sudoku[boxes[0] as usize])
                    & consts::SUDOKU_VALUES_TOTAL);
            match consts::OPTION_COUNT_CACHE[box_total as usize] {
                0 => {}
                1 => {
                    if value & box_total != 0 {
                        value &= box_total;
                        changed = true;
                    } else {
                        sudoku[square] = 0;
                        return false;
                    }
                }
                _ => {
                    sudoku[square] = 0;
                    return false;
                }
            }
            if changed {
                sudoku[square] =
                    value | ((sudoku[square] & consts::SUDOKU_TECHNIQUES_TOTAL) - 1024);
            }
        }
    }
    changed
}
