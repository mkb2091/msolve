include!(concat!(env!("OUT_DIR"), "/consts.rs"));

#[cfg(default)]
extern crate smallvec;

use std::convert::TryInto;

#[cfg(feature = "smallvec")]
type SudokuBackTrackingVec = smallvec::SmallVec<[Sudoku; 10]>;
#[cfg(not(feature = "smallvec"))]
type SudokuBackTrackingVec = Vec<Sudoku>;

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
const SCANNING_CUTOFF: u32 = 40;

/**
Iterator of the solutions of a sudoku
*/
pub struct SolutionIterator {
    routes: SudokuBackTrackingVec,
    step_count: usize,
}

impl SolutionIterator {
    #[inline]
    fn new(sudoku: Sudoku) -> Self {
        let mut routes = SudokuBackTrackingVec::with_capacity(10);
        if sudoku.cells.iter().all(|x| *x != 0) {
            routes.push(sudoku);
        }
        Self {
            routes,
            step_count: 0,
        }
    }
}

impl Iterator for SolutionIterator {
    type Item = Sudoku;
    /**
    Get the next solution
    */
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut state) = self.routes.pop() {
            self.step_count += 1;
            if let Ok(result) = state.handle_route(&mut self.routes) {
                return Some(result);
            }
        }
        None
    }
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

fn generate_masks_from_intersections(isec: [u16; 9]) -> ([u16; 9], [u16; 9]) {
    let only_1_1 = isec[0] & !((isec[1] | isec[2]) & (isec[3] | isec[6]));
    let only_1_2 = isec[1] & !((isec[0] | isec[2]) & (isec[4] | isec[7]));
    let only_1_3 = isec[2] & !((isec[0] | isec[1]) & (isec[5] | isec[8]));

    let only_2_1 = isec[3] & !((isec[4] | isec[5]) & (isec[0] | isec[6]));
    let only_2_2 = isec[4] & !((isec[3] | isec[5]) & (isec[1] | isec[7]));
    let only_2_3 = isec[5] & !((isec[3] | isec[4]) & (isec[2] | isec[8]));

    let only_3_1 = isec[6] & !((isec[7] | isec[8]) & (isec[0] | isec[3]));
    let only_3_2 = isec[7] & !((isec[6] | isec[8]) & (isec[1] | isec[4]));
    let only_3_3 = isec[8] & !((isec[6] | isec[7]) & (isec[2] | isec[5]));

    let resultant_mask = [
        !(only_1_2 | only_1_3 | only_2_1 | only_3_1),
        !(only_1_1 | only_1_3 | only_2_2 | only_3_2),
        !(only_1_1 | only_1_2 | only_2_3 | only_3_3),
        !(only_1_1 | only_2_2 | only_2_3 | only_3_1),
        !(only_1_2 | only_2_1 | only_2_3 | only_3_2),
        !(only_1_3 | only_2_1 | only_2_2 | only_3_3),
        !(only_1_1 | only_2_1 | only_3_2 | only_3_3),
        !(only_1_2 | only_2_2 | only_3_1 | only_3_3),
        !(only_1_3 | only_2_3 | only_3_1 | only_3_2),
    ];
    let only = [
        only_1_1, only_1_2, only_1_3, only_2_1, only_2_2, only_2_3, only_3_1, only_3_2, only_3_3,
    ];
    (resultant_mask, only)
}

/**
Structure that represents a sudoku
*/
#[derive(Copy, Clone)]
pub struct Sudoku {
    cells: [u16; 81],
    solved_squares: u128,
}

impl Sudoku {
    /**
    Remove the value at the chosen square from the set of options of each cell in the sudoku
    */
    #[inline(always)]
    fn apply_number(&mut self, square: usize) {
        debug_assert!(square < 81);
        if square >= 81 {
            unsafe { std::hint::unreachable_unchecked() }
        }
        let not_value = !self.cells[square];
        for i in &CELLS_TO_CHANGE[square] {
            self.cells[*i as usize] &= not_value;
        }

        debug_assert_eq!(self.cells[square], !not_value);
        self.solved_squares |= 1 << square;
    }

    /**
    Check what values the row, column and square it is in and compare them
    */
    fn hidden_singles(&mut self, square: usize) -> Result<bool, ()> {
        debug_assert!(square < 81);
        if square >= 81 {
            unsafe { std::hint::unreachable_unchecked() }
        }
        let value = self.cells[square];
        self.cells[square] = 0;
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
                            acc.0 | *self.cells.get_unchecked(row_start + i),
                            acc.1 | *self.cells.get_unchecked(column_start + i * 9),
                            acc.2 | *self.cells.get_unchecked(box_start + box_offset),
                        )
                    },
                );
                temp.0 & temp.1 & temp.2
            };
        if needed == 0 {
            self.cells[square] = value;
            Ok(false) // Don't yet know enough information to determine which value it must be
        } else if (value & needed).is_power_of_two() {
            self.cells[square] = value & needed;
            Ok(needed != value) // It can be the value it is needed to be
        } else {
            Err(()) // It has to be multiple different values, sudoku cannot be solved
        }
    }

    fn scan(&mut self) -> bool {
        let mut sudoku = self.cells;
        let mut sudoku_check = SUDOKU_MAX;
        for floor_number in (0..3).map(|x| x * 27) {
            let mut intersections = [0_u16; 9]; // Intersection
            for i in 0..9 {
                intersections[i] = sudoku[floor_number + i * 3]
                    | sudoku[floor_number + i * 3 + 1]
                    | sudoku[floor_number + i * 3 + 2];
            }
            let (resultant_mask, only) = generate_masks_from_intersections(intersections);

            let mut temp_total = 0;
            for (i, (row, only_row)) in resultant_mask.iter().zip(only.iter()).enumerate() {
                temp_total |= row;
                let row = row & [SUDOKU_MAX, *only_row][(only_row.count_ones() == 3) as usize];
                sudoku[floor_number + i * 3] &= row;
                sudoku[floor_number + i * 3 + 1] &= row;
                sudoku[floor_number + i * 3 + 2] &= row;
            }
            sudoku_check &= temp_total;
        }
        if sudoku_check != SUDOKU_MAX {
            return false;
        }
        for tower_number in (0..3).map(|x| x * 3) {
            let mut intersections = [0_u16; 9]; // Intersection
            for column in 0..3 {
                for layer in 0..3 {
                    intersections[column * 3 + layer] = sudoku[tower_number + layer * 27 + column]
                        | sudoku[tower_number + layer * 27 + column + 9]
                        | sudoku[tower_number + layer * 27 + column + 18]
                }
            }
            let (resultant_mask, only) = generate_masks_from_intersections(intersections);

            let mut temp_total = 0;

            for column_number in 0..3 {
                for layer in 0..3 {
                    let i = column_number * 3 + layer;
                    let column = resultant_mask[i];
                    let only_column = only[i];
                    temp_total |= column;
                    let column = column
                        & [SUDOKU_MAX, only_column][(only_column.count_ones() == 3) as usize];
                    sudoku[tower_number + layer * 27 + column_number] &= column;
                    sudoku[tower_number + layer * 27 + column_number + 9] &= column;
                    sudoku[tower_number + layer * 27 + column_number + 18] &= column;
                }
            }
            sudoku_check &= temp_total;
        }
        self.cells = sudoku;
        sudoku_check == SUDOKU_MAX
    }
    /**
    Perform a single iteration solving
    Call hidden_singles for each unsolved cell, and call apply_number for each newly solved cell\
    Select unsolved cell with least possible values
    For each possible value, clone the sudoku state, set the cell to the value and add to the state list
    */
    fn handle_route(&mut self, routes: &mut SudokuBackTrackingVec) -> Result<Self, ()> {
        if self.solved_squares.count_ones() == 81 {
            return Ok(*self);
        }
        let mut min: (usize, u32) = (0, std::u32::MAX);
        let mut temp = !self.solved_squares;
        loop {
            let square = get_last_digit(&mut temp);
            if square >= 81 {
                break;
            }
            if self.cells[square] == 0 {
                return Err(());
            }
            if self.cells[square].is_power_of_two() || self.hidden_singles(square)? {
                if self.solved_squares.count_ones() == 80 {
                    return Ok(*self);
                }
                self.apply_number(square);
            } else {
                let possible_values = self.cells[square].count_ones();
                if possible_values < min.1 {
                    min = (square, possible_values);
                }
            }
        }
        debug_assert!(min.1 <= 9);
        if self.solved_squares.count_ones() >= SCANNING_CUTOFF || (self.scan()) {
            let mut value = self.cells[min.0];
            while value != 0 {
                let i = value.trailing_zeros();
                value -= 1 << i;
                let mut new = *self;
                new.cells[min.0] = 1 << i;
                new.apply_number(min.0);
                routes.push(new);
            }
        }
        Err(())
    }

    /**
    Convert the sudoku into a [u8; 81] containing the numerical form of each solved square
    */
    pub fn to_array(&self) -> [u8; 81] {
        let mut array: [u8; 81] = [0; 81];
        for (square, processed) in self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, &value)| value.is_power_of_two())
        {
            array[square] = processed.trailing_zeros() as u8 + 1;
        }
        array
    }
    /**
    Returns an iterator over all solutions
    */
    #[inline]
    pub fn iter(self) -> SolutionIterator {
        SolutionIterator::new(self)
    }
    /**
    Get the first solution.
    */
    #[inline]
    pub fn solve(self) -> Option<Self> {
        self.iter().next()
    }

    /**
    Returns the first solution if it is uniquely solvable, otherwise returns None
    */
    #[inline]
    pub fn solve_unique(self) -> Option<Self> {
        let mut iterator = self.iter();
        iterator.next().xor(iterator.next())
    }
    /**
    Counts the number of solutions, up to maximum of n
    */
    #[inline]
    pub fn count_solutions(self, n: usize) -> usize {
        self.iter().take(n).count()
    }

    /**
    Check whether the sudoku has exactly one solution without returning the solution
    */
    #[inline]
    pub fn has_single_solution(self) -> bool {
        self.count_solutions(2) == 1
    }

    /**
    Returns an empty sudoku grid, alternative to Sudoku::from([0; 81]) or Sudoku::from(vec![])
    */
    #[inline]
    pub const fn empty() -> Self {
        Self {
            cells: [SUDOKU_MAX; 81],
            solved_squares: 0,
        }
    }
    /**
    Returns the number of steps to find the first solution, approximately proportional to difficulty
    */
    #[inline]
    pub fn solve_difficulty(self) -> usize {
        let mut iter = self.iter();
        iter.next();
        iter.step_count
    }
    /**
    Returns the number of steps to find the first two solutions, approximately proportional to difficulty
    */
    #[inline]
    pub fn solve_unique_difficulty(self) -> usize {
        let mut iter = self.iter();
        iter.next();
        iter.next();
        iter.step_count
    }
}

impl<T: TryInto<usize> + Copy> From<&[T]> for Sudoku {
    fn from(sudoku_array: &[T]) -> Self {
        let mut sudoku = Self::empty();
        for (i, item) in sudoku_array
            .iter()
            .enumerate()
            .take(81)
            .filter_map(|(i, item)| {
                (*item)
                    .try_into()
                    .ok()
                    .and_then(|x| x.checked_sub(1))
                    .filter(|x| *x <= 8)
                    .map(|x| (i, x))
            })
        {
            sudoku.cells[i] = 1 << item;
            sudoku.apply_number(i);
        }
        sudoku.scan();
        sudoku
    }
}
impl<T: TryInto<usize> + Copy> From<&[T; 81]> for Sudoku {
    #[inline]
    fn from(sudoku_array: &[T; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<usize> + Copy> From<[T; 81]> for Sudoku {
    #[inline]
    fn from(sudoku_array: [T; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<usize> + Copy> From<Vec<T>> for Sudoku {
    #[inline]
    fn from(sudoku_array: Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<usize> + Copy> From<&Vec<T>> for Sudoku {
    #[inline]
    fn from(sudoku_array: &Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}

impl From<&str> for Sudoku {
    fn from(sudoku_str: &str) -> Self {
        let mut sudoku = Self::empty();
        for (i, int) in sudoku_str
            .chars()
            .enumerate()
            .take(81)
            .filter_map(|(i, character)| {
                character
                    .to_digit(10)
                    .and_then(|int| int.checked_sub(1))
                    .map(|int| (i, int))
            })
        {
            sudoku.cells[i] = 1 << int;
            sudoku.apply_number(i);
        }
        sudoku.scan();
        sudoku
    }
}
impl From<String> for Sudoku {
    #[inline]
    fn from(sudoku_str: String) -> Self {
        Self::from(&sudoku_str[..])
    }
}
impl From<&String> for Sudoku {
    #[inline]
    fn from(sudoku_str: &String) -> Self {
        Self::from(&sudoku_str[..])
    }
}
