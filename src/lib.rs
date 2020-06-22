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
Remove the value of at the chosen square from the set of options of each cell in the sudoku
*/
fn apply_number(sudoku: &mut Sudoku, square: usize) {
    debug_assert!(square < 81);
    #[cfg(not(debug_assertions))]
    if square >= 81 {
        unsafe { std::hint::unreachable_unchecked() }
    }
    let value = sudoku[square];
    let not_value = !value;
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
    debug_assert!(square < 81);
    #[cfg(not(debug_assertions))]
    if square >= 81 {
        unsafe { std::hint::unreachable_unchecked() }
    }
    let value = sudoku[square];
    sudoku[square] = 0;
    let row_start = square / 9 * 9;
    let column_start = square % 9;
    let box_start = square / 3 % 3 * 3 + square / 27 * 27;
    debug_assert!(row_start + 8 < 81);
    debug_assert!(column_start + 72 < 81);
    debug_assert!(box_start + 20 < 81);
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

/**
Apply pointing pairs technique.
For each box, determine which values can only be in a single intersection,
and then remove them from the house the intersection is in
*/
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
        let only_row1 = !row1 | row2 | row3;
        let only_row2 = row1 | !row2 | row3;
        let only_row3 = row1 | row2 | !row3;
        let rows = [only_row1, only_row2, only_row3];
        for (row_number, row) in rows.iter().enumerate() {
            for i in 0..9 {
                sudoku[row_start + row_number * 9 + i] &= row;
            }
        }
        let column1 = box_old[0] | box_old[3] | box_old[6];
        let column2 = box_old[1] | box_old[4] | box_old[7];
        let column3 = box_old[2] | box_old[5] | box_old[8];
        let only_column1 = !column1 | column2 | column3;
        let only_column2 = column1 | !column2 | column3;
        let only_column3 = column1 | column2 | !column3;
        let columns = [only_column1, only_column2, only_column3];
        for (column_number, column) in columns.iter().enumerate() {
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

fn box_line_reduction(sudoku_ref: &mut Sudoku) -> bool {
    let mut sudoku = *sudoku_ref;
    let mut sudoku_check = SUDOKU_MAX;
    for floor_number in (0..3).map(|x| x * 27) {
        let mut intersection = [0_u16; 9]; // Intersection
        for i in 0..9 {
            intersection[i] = sudoku[floor_number + i * 3]
                | sudoku[floor_number + i * 3 + 1]
                | sudoku[floor_number + i * 3 + 2];
        }
        // Rows
        let only_row_1_1 = intersection[0] & !(intersection[1] | intersection[2]);
        let only_row_1_2 = intersection[1] & !(intersection[0] | intersection[2]);
        let only_row_1_3 = intersection[2] & !(intersection[0] | intersection[1]);

        let only_row_2_1 = intersection[3] & !(intersection[4] | intersection[5]);
        let only_row_2_2 = intersection[4] & !(intersection[3] | intersection[5]);
        let only_row_2_3 = intersection[5] & !(intersection[3] | intersection[4]);

        let only_row_3_1 = intersection[6] & !(intersection[7] | intersection[8]);
        let only_row_3_2 = intersection[7] & !(intersection[6] | intersection[8]);
        let only_row_3_3 = intersection[8] & !(intersection[6] | intersection[7]);

        let resultant_mask = [
            !(only_row_1_2 | only_row_1_3 | only_row_2_1 | only_row_3_1),
            !(only_row_1_1 | only_row_1_3 | only_row_2_2 | only_row_3_2),
            !(only_row_1_1 | only_row_1_2 | only_row_2_3 | only_row_3_3),
            !(only_row_1_1 | only_row_2_2 | only_row_2_3 | only_row_3_1),
            !(only_row_1_2 | only_row_2_1 | only_row_2_3 | only_row_3_2),
            !(only_row_1_3 | only_row_2_1 | only_row_2_2 | only_row_3_3),
            !(only_row_1_1 | only_row_2_1 | only_row_3_2 | only_row_3_3),
            !(only_row_1_2 | only_row_2_2 | only_row_3_1 | only_row_3_3),
            !(only_row_1_3 | only_row_2_3 | only_row_3_1 | only_row_3_2),
        ];

        let mut temp_total = 0;
        for (i, row) in resultant_mask.iter().enumerate() {
            temp_total |= row;
            sudoku[floor_number + i * 3] &= row;
            sudoku[floor_number + i * 3 + 1] &= row;
            sudoku[floor_number + i * 3 + 2] &= row;
        }
        sudoku_check &= temp_total;

        let only_rows = [
            only_row_1_1,
            only_row_1_2,
            only_row_1_3,
            only_row_2_1,
            only_row_2_2,
            only_row_2_3,
            only_row_3_1,
            only_row_3_2,
            only_row_3_3,
        ];

        for (i, row) in only_rows.iter().enumerate() {
            if row.count_ones() == 3 {
                sudoku[floor_number + i * 3] &= row;
                sudoku[floor_number + i * 3 + 1] &= row;
                sudoku[floor_number + i * 3 + 2] &= row;
            }
        }
    }
    *sudoku_ref = sudoku;
    sudoku_check == SUDOKU_MAX
}

/**
Remove and return the last set bit in a u128
*/
#[inline(always)]
fn get_last_digit(x: &mut u128) -> usize {
    let value = x.trailing_zeros();
    *x -= 1 << value;
    value as usize
}

/**
Perform a single iteration solving
Call hidden_singles for each unsolved cell, and call apply_number for each newly solved cell\
Select unsolved cell with least possible values
For each possible value, clone the sudoku state, set the cell to the value and add to the state list
*/
fn handle_route(
    mut route: Sudoku,
    mut solved_squares: u128,
    routes: &mut SudokuBackTrackingVec,
) -> Result<Sudoku, ()> {
    if solved_squares.count_ones() == 81 {
        return Ok(route);
    }
    let mut min: (usize, u32) = (0, std::u32::MAX);
    let mut temp = !solved_squares;
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
    if solved_squares.count_ones() >= POINTING_PAIRS_CUTOFF
        || (pointing_pairs(&mut route) && box_line_reduction(&mut route))
    {
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
    /**
    Initialise the solver
    */
    fn new(mut sudoku: Sudoku) -> Self {
        let mut solved_squares = 0;
        for square in 0..81 {
            if sudoku[square].is_power_of_two() {
                solved_squares |= 1 << square;
                apply_number(&mut sudoku, square as usize);
            }
        }
        let mut routes = SudokuBackTrackingVec::with_capacity(10);
        routes.push((sudoku, solved_squares));
        SolutionIterator { routes }
    }
}

impl Iterator for SolutionIterator {
    type Item = SudokuStruct;
    /**
    Get the next solution
    */
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
    /**
    Convert the sudoku into a [u8; 81] containing the numerical form of each solved square
    */
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
    /**
    Get the first solution.
    */
    pub fn iter(self) -> SolutionIterator {
        SolutionIterator::new(self.sudoku)
    }
    pub fn solve(self) -> Option<SudokuStruct> {
        self.iter().next()
    }

    /**
    Returns the first solution if it is uniquely solvable, otherwise returns None
    */
    pub fn solve_unique(self) -> Option<SudokuStruct> {
        let mut iterator = self.iter();
        let result = iterator.next();
        if iterator.next().is_none() {
            return result;
        }
        None
    }
    /**
    Counts the number of solutions, up to maximum of n
    */
    pub fn count_solutions(self, n: usize) -> usize {
        self.iter().take(n).count()
    }
    pub fn has_single_solution(self) -> bool {
        self.count_solutions(2) == 1
    }
}

use std::convert::TryInto;

impl<T: TryInto<u8> + Copy> From<&[T]> for SudokuStruct {
    fn from(sudoku_array: &[T]) -> Self {
        let mut sudoku: [u16; 81] = [SUDOKU_MAX; 81];
        for (item, pointer) in sudoku_array
            .iter()
            .zip(sudoku.iter_mut())
            .filter_map(|(item, pointer)| {
                if let Ok(x) = (*item).try_into() {
                    Some((x, pointer))
                } else {
                    None
                }
            })
            .filter(|(item, _)| *item != 0 && *item <= 9)
        {
            *pointer = 1 << (item - 1);
        }
        Self { sudoku }
    }
}
impl<T: TryInto<u8> + Copy> From<&[T; 81]> for SudokuStruct {
    fn from(sudoku_array: &[T; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<u8> + Copy> From<[T; 81]> for SudokuStruct {
    fn from(sudoku_array: [T; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<u8> + Copy> From<Vec<T>> for SudokuStruct {
    fn from(sudoku_array: Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<u8> + Copy> From<&Vec<T>> for SudokuStruct {
    fn from(sudoku_array: &Vec<T>) -> Self {
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
