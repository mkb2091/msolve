use crate::consts;

pub fn naked_n(sudoku: &mut [u16; 81]) -> bool {
    let mut changed: bool = false;
    for square in 0..81 {
        let value = sudoku[square] & consts::SUDOKU_VALUES_TOTAL;
        if sudoku[square] & 512 == 512 {
            // Naked singles
            if consts::OPTION_COUNT_CACHE[value as usize] == 1 {
                changed = true;
                let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
                for house in [rows, columns, boxes].iter() {
                    for pos in house.iter() {
                        if sudoku[*pos as usize] & value != 0 {
                            sudoku[*pos as usize] =
                                (sudoku[*pos as usize] ^ (value)) | consts::SUDOKU_TECHNIQUES_TOTAL;
                        }
                    }
                }
            // Naked Pairs
            } else if consts::OPTION_COUNT_CACHE[value as usize] == 2 {
                let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
                for house in [rows, columns, boxes].iter() {
                    for pos in house.iter() {
                        if sudoku[*pos as usize] & consts::SUDOKU_VALUES_TOTAL == value {
                            changed = true;
                            for pos2 in house.iter() {
                                if pos2 != pos  && (sudoku[*pos2 as usize] & value != 0){
                                    sudoku[*pos2 as usize] = (sudoku[*pos2 as usize] ^ (value))
                                        | consts::SUDOKU_TECHNIQUES_TOTAL;
                                }
                            }
                        }
                    }
                }
            // Naked Triples
            // Currently only works for 3 identical cells
            } else if consts::OPTION_COUNT_CACHE[value as usize] == 3 {
                let (rows, columns, boxes) = consts::PRECOMPUTED_INDEXES[square];
                for house in [rows, columns, boxes].iter() {
                    for pos in house.iter() {
                        if sudoku[*pos as usize] & consts::SUDOKU_VALUES_TOTAL == value {
                            for pos2 in house.iter() {
                                if pos2 != pos
                                    && sudoku[*pos2 as usize] & consts::SUDOKU_VALUES_TOTAL == value
                                {
                                    changed = true;
                                    for pos3 in house.iter() {
                                        if pos3 != pos && pos3 != pos2 && (sudoku[*pos3 as usize] & value != 0) {
                                            sudoku[*pos3 as usize] = (sudoku[*pos3 as usize]
                                                ^ (value))
                                                | consts::SUDOKU_TECHNIQUES_TOTAL;
                                        }
                                    }
                                }
                            }
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
