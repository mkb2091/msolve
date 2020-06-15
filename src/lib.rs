#[cfg(default)]
extern crate smallvec;

/**
Represents a sudoku grid, with each square representing which possible numbers it could be
*/
type Sudoku = [u16; 81];

type SudokuState = (Sudoku, u128);
#[cfg(default)]
type SudokuBackTrackingVec = smallvec::SmallVec<[SudokuState; 10]>;
#[cfg(not(default))]
type SudokuBackTrackingVec = Vec<SudokuState>;

/** Max 9 bit number */
const SUDOKU_MAX: u16 = (1 << 9) - 1;

/*
After solving this many squares, do not use pointing pairs
For top 2465, 33 is best
For top 2465 unique, 35 is best
For sudoku17, 41 is best
For sudoku17 unique, 42 is best
For empty_n, lower is better, though limited difference between values below 55
*/
const POINTING_PAIRS_CUTOFF: u32 = 40;

/**
To be called when there is only one possible number
*/
pub fn apply_number(sudoku: &mut Sudoku, square: usize) {
    let value = sudoku[square];
    let not_value = SUDOKU_MAX - value;
    let column_start = square % 9;
    let row_start = square - column_start;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    unsafe {
        for (i, box_offset) in [20, 19, 18, 11, 10, 9, 2, 1, 0].iter().enumerate() {
            *sudoku.get_unchecked_mut(row_start + i) &= not_value;
            *sudoku.get_unchecked_mut(column_start + i * 9) &= not_value;
            *sudoku.get_unchecked_mut(box_start + box_offset) &= not_value;
        }
    }
    sudoku[square] = value;
}

/**
Check what values the row, column and square it is in and compare them
*/
fn hidden_singles(sudoku: &mut Sudoku, square: usize) -> Result<bool, ()> {
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
    } else if (value & needed).is_power_of_two() {
        sudoku[square] = value & needed;
        Ok(needed != value) // It can be the value it is needed to be
    } else {
        Err(()) // It has to be multiple different values, sudoku cannot be solved
    }
}

fn pointing_pairs(sudoku_ref: &mut Sudoku) -> bool {
    let mut sudoku = *sudoku_ref;
    let mut sudoku_check = SUDOKU_MAX;
    for &box_start in [0, 3, 6, 27, 30, 33, 54, 57, 60].iter() {
        let row_start = box_start / 9 * 9;
        let column_start = box_start % 9;
        let box_old = [
            sudoku[box_start],
            sudoku[box_start + 1],
            sudoku[box_start + 2],
            sudoku[box_start + 9],
            sudoku[box_start + 10],
            sudoku[box_start + 11],
            sudoku[box_start + 18],
            sudoku[box_start + 19],
            sudoku[box_start + 20],
        ];
        let row1 = box_old[0] | box_old[1] | box_old[2];
        let row2 = box_old[3] | box_old[4] | box_old[5];
        let row3 = box_old[6] | box_old[7] | box_old[8];
        let only_row1 = row1 & (SUDOKU_MAX - (row2 | row3));
        let only_row2 = row2 & (SUDOKU_MAX - (row1 | row3));
        let only_row3 = row3 & (SUDOKU_MAX - (row1 | row2));
        let rows = [only_row1, only_row2, only_row3];
        for row_number in 0..3 {
            let row = SUDOKU_MAX - rows[row_number];
            for i in 0..9 {
                sudoku[row_start + row_number * 9 + i] &= row;
            }
        }
        let column1 = box_old[0] | box_old[3] | box_old[6];
        let column2 = box_old[1] | box_old[4] | box_old[7];
        let column3 = box_old[2] | box_old[5] | box_old[8];
        let only_column1 = column1 & (SUDOKU_MAX - (column2 | column3));
        let only_column2 = column2 & (SUDOKU_MAX - (column1 | column3));
        let only_column3 = column3 & (SUDOKU_MAX - (column1 | column2));
        let columns = [only_column1, only_column2, only_column3];
        for column_number in 0..3 {
            let column = SUDOKU_MAX - columns[column_number];
            for i in 0..9 {
                sudoku[column_start + column_number + i * 9] &= column;
            }
        }
        sudoku[box_start] = box_old[0];
        sudoku[box_start + 1] = box_old[1];
        sudoku[box_start + 2] = box_old[2];
        sudoku[box_start + 9] = box_old[3];
        sudoku[box_start + 10] = box_old[4];
        sudoku[box_start + 11] = box_old[5];
        sudoku[box_start + 18] = box_old[6];
        sudoku[box_start + 19] = box_old[7];
        sudoku[box_start + 20] = box_old[8];
        sudoku_check &= column1 | column2 | column3;
    }
    *sudoku_ref = sudoku;
    sudoku_check == SUDOKU_MAX
}

#[inline(always)]
fn get_last_digit(x: &mut u128) -> usize {
    let value = x.trailing_zeros();
    *x -= 1 << value;
    value as usize
}

fn handle_route(
    mut route: Sudoku,
    mut solved_squares: u128,
    routes: &mut SudokuBackTrackingVec,
) -> Result<Sudoku, ()> {
    if solved_squares.count_ones() == 81 {
        return Ok(route);
    }
    let mut min: (usize, u32) = (0, std::u32::MAX);
    let mut temp = std::u128::MAX - solved_squares;
    loop {
        let square = get_last_digit(&mut temp);
        if square >= 81 {
            break;
        }
        if route[square] == 0 {
            return Err(());
        }
        if route[square].is_power_of_two() {
            if solved_squares.count_ones() == 80 {
                return Ok(route);
            }
            apply_number(&mut route, square as usize);
            solved_squares |= 1 << square;
            continue;
        }
        if let Ok(changed) = hidden_singles(&mut route, square as usize) {
            debug_assert_eq!(changed || route[square].is_power_of_two(), changed);
            if changed {
                if solved_squares.count_ones() == 80 {
                    return Ok(route);
                }
                apply_number(&mut route, square as usize);
                solved_squares |= 1 << square;
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
    debug_assert!(min.1 <= 9);
    let mut value = route[min.0];
    if solved_squares.count_ones() >= POINTING_PAIRS_CUTOFF || pointing_pairs(&mut route) {
        solved_squares |= 1 << min.0;
        while value != 0 {
            let i = value.trailing_zeros();
            value -= 1 << i;
            let mut new = route;
            new[min.0] = 1 << i;
            apply_number(&mut new, min.0);
            routes.push((new, solved_squares));
        }
    }
    Err(())
}

pub struct SolutionIterator {
    routes: SudokuBackTrackingVec,
}

impl SolutionIterator {
    fn new(mut sudoku: Sudoku) -> Self {
        let mut solved_squares = 0;
        for square in 0..81 {
            if sudoku[square].is_power_of_two() {
                solved_squares |= 1 << square;
                apply_number(&mut sudoku, square as usize);
            }
        }
        #[cfg(default)]
        let routes: SudokuBackTrackingVec = smallvec::smallvec![(sudoku, solved_squares)];
        #[cfg(not(default))]
        let routes: SudokuBackTrackingVec = vec![(sudoku, solved_squares)];
        SolutionIterator { routes }
    }
}

impl Iterator for SolutionIterator {
    type Item = SudokuStruct;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((route, solved_squares)) = self.routes.pop() {
            if let Ok(result) = handle_route(route, solved_squares, &mut self.routes) {
                return Some(SudokuStruct { sudoku: result });
            }
        }
        None
    }
}

pub struct SudokuStruct {
    sudoku: Sudoku,
}

impl SudokuStruct {
    pub fn to_array(&self) -> [u8; 81] {
        let mut array: [u8; 81] = [0; 81];
        for (square, processed) in self
            .sudoku
            .iter()
            .enumerate()
            .filter(|(_, &value)| value.is_power_of_two())
        {
            array[square] = processed.trailing_zeros() as u8 + 1;
        }
        array
    }
    pub fn solve(self) -> Option<SudokuStruct> {
        SolutionIterator::new(self.sudoku).next()
    }

    pub fn solve_unique(self) -> Option<SudokuStruct> {
        let mut iterator = SolutionIterator::new(self.sudoku);
        if let Some(result) = iterator.next() {
            if iterator.next().is_none() {
                return Some(result)
            }
        }
        None
    }

    pub fn count_solutions(self, n: usize) -> usize {
        SolutionIterator::new(self.sudoku).take(n).count()
    }
}

impl From<&[u8]> for SudokuStruct {
    fn from(sudoku_array: &[u8]) -> Self {
        let mut sudoku: [u16; 81] = [SUDOKU_MAX; 81];
        for (item, pointer) in sudoku_array
            .iter()
            .zip(sudoku.iter_mut())
            .filter(|(item, _)| **item != 0)
        {
            *pointer = 1 << (item - 1);
        }
        Self { sudoku }
    }
}
impl From<&[u8; 81]> for SudokuStruct {
    fn from(sudoku_array: &[u8; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}

impl From<&str> for SudokuStruct {
    fn from(sudoku_str: &str) -> Self {
        let mut sudoku: [u16; 81] = [SUDOKU_MAX; 81];
        for (square, character) in sudoku.iter_mut().zip(sudoku_str.chars()) {
            if let Some(int) = character.to_digit(10) {
                if int != 0 {
                    *square = 1 << (int - 1);
                }
            }
        }
        Self { sudoku }
    }
}
impl From<String> for SudokuStruct {
    fn from(sudoku_str: String) -> Self {
        Self::from(&sudoku_str[..])
    }
}
impl From<&String> for SudokuStruct {
    fn from(sudoku_str: &String) -> Self {
        Self::from(&sudoku_str[..])
    }
}
