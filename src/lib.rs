/** Max 9 bit number */
pub const SUDOKU_MAX: u16 = 512 - 1;

/**
Represents a sudoku grid, with each square representing which possible numbers it could be
*/
pub type Sudoku = [u16; 81];

/**
To be called when there is only one possible number
*/
pub fn apply_number(sudoku: &mut Sudoku, square: usize) {
    let value = sudoku[square];
    let not_value = SUDOKU_MAX - value;
    let column_start = square % 9;
    let row_start = square - column_start;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    sudoku[row_start + 8] &= not_value;
    sudoku[row_start + 7] &= not_value;
    sudoku[row_start + 6] &= not_value;
    sudoku[row_start + 5] &= not_value;
    sudoku[row_start + 4] &= not_value;
    sudoku[row_start + 3] &= not_value;
    sudoku[row_start + 2] &= not_value;
    sudoku[row_start + 1] &= not_value;
    sudoku[row_start] &= not_value;

    sudoku[column_start + 72] &= not_value;
    sudoku[column_start + 63] &= not_value;
    sudoku[column_start + 54] &= not_value;
    sudoku[column_start + 45] &= not_value;
    sudoku[column_start + 36] &= not_value;
    sudoku[column_start + 27] &= not_value;
    sudoku[column_start + 18] &= not_value;
    sudoku[column_start + 9] &= not_value;
    sudoku[column_start] &= not_value;

    sudoku[box_start + 20] &= not_value;
    sudoku[box_start + 19] &= not_value;
    sudoku[box_start + 18] &= not_value;
    sudoku[box_start + 11] &= not_value;
    sudoku[box_start + 10] &= not_value;
    sudoku[box_start + 9] &= not_value;
    sudoku[box_start + 2] &= not_value;
    sudoku[box_start + 1] &= not_value;
    sudoku[box_start] &= not_value;
    sudoku[square] = value;
}

/**
Check what values the row, column and square it is in and compare them
*/
pub fn hidden_singles(sudoku: &mut Sudoku, square: usize) -> Result<bool, ()> {
    let value = sudoku[square];
    sudoku[square] = 0;
    let row_start = square / 9 * 9;
    let column_start = square % 9;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    assert!(row_start + 8 < 81);
    assert!(column_start + 72 < 81);
    assert!(box_start + 20 < 81);
    let needed = SUDOKU_MAX
        - unsafe {
            let temp = [20, 19, 18, 11, 10, 9, 2, 1, 0].iter().enumerate().fold(
                (0, 0, 0),
                |acc, (i, box_offset)| {
                    (
                        acc.0 | *sudoku.get_unchecked(row_start + i),
                        acc.1 | *sudoku.get_unchecked(column_start + i * 9),
                        acc.2 | *sudoku.get_unchecked(box_start + box_offset),
                    )
                },
            );
            temp.0 & temp.1 & temp.2
        };
    if needed == 0 {
        sudoku[square] = value;
        Ok(false) // Don't yet know enough information to determine which value it must be
    } else if needed.is_power_of_two() {
        if value & needed != 0 {
            sudoku[square] = value & needed;
            Ok(needed != value) // It can be the value it is needed to be
        } else {
            Err(()) // It can't be the value it is needed to be, sudoku cannot be solved
        }
    } else {
        Err(()) // It has to be multiple different values, sudoku cannot be solved
    }
}

fn to_sudoku(sudoku: &[u8; 81]) -> Sudoku {
    let mut options: [u16; 81] = [SUDOKU_MAX; 81];
    for (i, item) in sudoku.iter().enumerate() {
        if *item != 0 {
            options[i] = 1 << (item - 1);
        }
    }
    options
}

fn str_to_sudoku(sudoku_str: &str) -> Sudoku {
    let mut sudoku = [0; 81];
    for (square, char) in sudoku.iter_mut().zip(sudoku_str.chars()) {
        *square = match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => 0,
        };
    }
    to_sudoku(&sudoku)
}

fn from_sudoku(sudoku: &Sudoku) -> [u8; 81] {
    let mut array: [u8; 81] = [0; 81];
    for (square, processed) in sudoku
        .iter()
        .enumerate()
        .map(|(square, &value)| (square, value & SUDOKU_MAX))
    {
        if let Some((i, _)) = (0..).map(|x| (x, 1 << x)).find(|(_, v)| processed == *v) {
            array[square] = i as u8 + 1;
        }
    }
    array
}

#[inline(always)]
fn get_last_digit(x: &mut u128) -> usize {
    let value = x.trailing_zeros();
    *x -= 1 << value;
    value as usize
}

#[derive(Clone, Copy)]
pub struct Solver {
    changed_squares_from_apply: [u128; 81],
}

impl Solver {
    pub fn new() -> Solver {
        let mut changed_squares_from_apply = [0; 81];
        for i in 0..81 {
            let mut square: u128 = 0;
            let column_start = i % 9;
            let row_start = i - column_start;
            let box_start = i / 3 % 3 * 3 + i / 27 * 27;
            for x in 0..9 {
                square |= 1 << (row_start + x);
                square |= 1 << (column_start + 9 * x);
            }
            square |= 1 << (box_start);
            square |= 1 << (box_start + 1);
            square |= 1 << (box_start + 2);
            square |= 1 << (box_start + 9);
            square |= 1 << (box_start + 10);
            square |= 1 << (box_start + 11);
            square |= 1 << (box_start + 18);
            square |= 1 << (box_start + 19);
            square |= 1 << (box_start + 20);
            square &= std::u128::MAX - (1 << i);
            changed_squares_from_apply[i] = square;
        }
        Solver {
            changed_squares_from_apply,
        }
    }

    fn handle_route(
        &self,
        mut route: Sudoku,
        mut changed_squares: u128,
        mut solved_squares: u128,
        routes: &mut Vec<(Sudoku, u128, u128)>,
    ) -> Result<Sudoku, ()> {
        let mut temp = std::u128::MAX - solved_squares;
        let mut min: (usize, u32) = (0, std::u32::MAX);
        loop {
            if changed_squares != 0 {
                while changed_squares != 0 {
                    let square = get_last_digit(&mut changed_squares);
                    if route[square].is_power_of_two() {
                        if solved_squares.count_ones() == 80 {
                            return Ok(route);
                        }
                        apply_number(&mut route, square as usize);
                        solved_squares |= 1 << square;
                        changed_squares |= self.changed_squares_from_apply[square];
                        changed_squares &= std::u128::MAX - solved_squares;
                    } else if route[square] == 0 {
                        return Err(());
                    }
                }
                temp = std::u128::MAX - solved_squares;
                min = (0, std::u32::MAX);
            }
            while temp != 0 {
                let square = get_last_digit(&mut temp);
                if square >= 81 {
                    // Iterated though all squares without finding a value to change
                    debug_assert!(min.1 != std::u32::MAX);
                    let value = route[min.0];
                    for i in 0..9 {
                        if value & (1 << i) != 0 {
                            let mut new = route;
                            new[min.0] = 1 << i;
                            routes.push((new, changed_squares | (1 << min.0), solved_squares));
                        }
                    }
                    return Err(());
                }
                if route[square] == 0 {
                    return Err(());
                }
                if let Ok(changed) = hidden_singles(&mut route, square as usize) {
                    if changed {
                        //changed_squares |= 1 << square;
                        apply_number(&mut route, square as usize);
                        solved_squares |= 1 << square;
                        changed_squares |= self.changed_squares_from_apply[square];
                        changed_squares &= std::u128::MAX - solved_squares;
                    } else {
                        let possible_values = route[square].count_ones();
                        if possible_values < min.1 {
                            min = (square, possible_values);
                        }
                    }
                } else {
                    return Err(());
                }
            }
        }
    }

    pub fn solve(&self, sudoku: Sudoku) -> Option<Sudoku> {
        let mut sudoku = sudoku;
        let mut changed_squares = 0;
        let mut solved_squares = 0;
        for square in 0..81 {
            if sudoku[square].is_power_of_two() {
                solved_squares |= 1 << square;
                apply_number(&mut sudoku, square as usize);
                changed_squares |= self.changed_squares_from_apply[square];
                changed_squares &= std::u128::MAX - solved_squares;
            }
        }
        let mut routes: Vec<(Sudoku, u128, u128)> = vec![(sudoku, changed_squares, solved_squares)];
        // (Sudoku, changed squares bitset, solved_squared)
        while let Some((route, changed_squares, solved_squares)) = routes.pop() {
            if let Ok(result) =
                self.handle_route(route, changed_squares, solved_squares, &mut routes)
            {
                return Some(result);
            }
        }

        None
    }
    pub fn solve_array(&self, sudoku: &[u8; 81]) -> Option<[u8; 81]> {
        if let Some(solution) = self.solve(to_sudoku(sudoku)) {
            Some(from_sudoku(&solution))
        } else {
            None
        }
    }
    pub fn solve_string(&self, sudoku: &str) -> Option<[u8; 81]> {
        if let Some(solution) = self.solve(str_to_sudoku(sudoku)) {
            Some(from_sudoku(&solution))
        } else {
            None
        }
    }
}
