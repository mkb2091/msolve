#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::*;

#[cfg(not(feature = "std"))]
use core::prelude::v1::*;

mod consts;

#[cfg(feature = "generate")]
pub mod gen;

#[cfg(all(not(feature = "std"), feature = "rand"))]
compile_error!("`std` feature is required for rand");

#[cfg(all(not(feature = "alloc"), feature = "smallvec"))]
compile_error!("`std` feature is required for smallvec");

#[cfg(not(feature = "alloc"))]
compile_error!("`alloc` feature is currently required");

pub mod solution_iterator;

use core::convert::From;
use core::convert::TryInto;
use core::str::FromStr;

#[derive(Default)]
struct DifficultyRecording {
    step_count: usize,
    apply_number_count: usize,
    scan_count: usize,
    hidden_single_count: usize,
}

impl solution_iterator::TechniqueRecording for DifficultyRecording {
    type Output = usize;
    fn record_step(&mut self, _: &Sudoku) {
        self.step_count += 1;
    }
    fn record_apply_number(&mut self, _: usize, _: &Sudoku) {
        self.apply_number_count += 1;
    }
    fn record_scan(&mut self, _: &Sudoku) {
        self.scan_count += 1;
    }
    fn record_hidden_single(&mut self, _: usize, _: &Sudoku) {
        self.hidden_single_count += 1;
    }
    fn get_recording(&self) -> usize {
        self.step_count + self.apply_number_count + self.scan_count + self.hidden_single_count
    }
}

#[cfg(feature = "alloc")]
#[derive(Default)]
struct FullRecording {
    techniques: Vec<(String, Sudoku)>,
}

#[cfg(feature = "alloc")]
impl solution_iterator::TechniqueRecording for FullRecording {
    type Output = Vec<(String, Sudoku)>;
    fn record_step(&mut self, _: &Sudoku) {}
    fn record_apply_number(&mut self, square: usize, state: &Sudoku) {
        let mut explanation = "Found naked single: R".to_string();
        explanation.push(b"123456789"[square / 9] as char);
        explanation.push('C');
        explanation.push(b"123456789"[square % 9] as char);
        self.techniques.push((explanation, *state))
    }
    fn record_scan(&mut self, state: &Sudoku) {
        self.techniques.push(("Scanned".to_string(), *state))
    }
    fn record_hidden_single(&mut self, square: usize, state: &Sudoku) {
        let mut explanation = "Found hidden single: R".to_string();
        explanation.push(b"123456789"[square / 9] as char);
        explanation.push('C');
        explanation.push(b"123456789"[square % 9] as char);
        self.techniques.push((explanation, *state))
    }
    fn get_recording(&self) -> Self::Output {
        let mut result = self.techniques.clone();
        result.dedup_by_key(|(_, sudoku)| *sudoku);
        result
    }
}

#[macro_export]
macro_rules! get_last_digit {
    ($x:ident, $value_type:ty) => {{
        let value = $x.trailing_zeros();
        $x -= 1 << value;
        value as $value_type
    }};
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
            unsafe { core::hint::unreachable_unchecked() }
        }
        let not_value = !self.cells[square];
        for i in &consts::CELLS_TO_CHANGE[square] {
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
            unsafe { core::hint::unreachable_unchecked() }
        }
        let value = self.cells[square];
        self.cells[square] = 0;
        let row_start = square / 9 * 9;
        let column_start = square % 9;
        let box_start = square / 3 % 3 * 3 + square / 27 * 27;
        debug_assert!(row_start + 8 < 81);
        debug_assert!(column_start + 72 < 81);
        debug_assert!(box_start + 20 < 81);
        let needed = consts::SUDOKU_MAX
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
        fn generate_masks_from_intersections(
            isec: [u16; 9],
            mut only: [u16; 9],
        ) -> ([u16; 9], [u16; 9]) {
            only[0] |= isec[0] & !((isec[1] | isec[2]) & (isec[3] | isec[6]));
            only[1] |= isec[1] & !((isec[0] | isec[2]) & (isec[4] | isec[7]));
            only[2] |= isec[2] & !((isec[0] | isec[1]) & (isec[5] | isec[8]));

            only[3] |= isec[3] & !((isec[4] | isec[5]) & (isec[0] | isec[6]));
            only[4] |= isec[4] & !((isec[3] | isec[5]) & (isec[1] | isec[7]));
            only[5] |= isec[5] & !((isec[3] | isec[4]) & (isec[2] | isec[8]));

            only[6] |= isec[6] & !((isec[7] | isec[8]) & (isec[0] | isec[3]));
            only[7] |= isec[7] & !((isec[6] | isec[8]) & (isec[1] | isec[4]));
            only[8] |= isec[8] & !((isec[6] | isec[7]) & (isec[2] | isec[5]));

            let resultant_mask = [
                !(only[1] | only[2] | only[3] | only[6]),
                !(only[0] | only[2] | only[4] | only[7]),
                !(only[0] | only[1] | only[5] | only[8]),
                !(only[0] | only[4] | only[5] | only[6]),
                !(only[1] | only[3] | only[5] | only[7]),
                !(only[2] | only[3] | only[4] | only[8]),
                !(only[0] | only[3] | only[7] | only[8]),
                !(only[1] | only[4] | only[6] | only[8]),
                !(only[2] | only[5] | only[6] | only[7]),
            ];
            (resultant_mask, only)
        }
        let mut sudoku = self.cells;
        let mut sudoku_check = consts::SUDOKU_MAX;
        for floor_number in (0..3).map(|x| x * 27) {
            let mut only = [0; 9];
            let mut intersections = [0_u16; 9]; // Intersection
            for i in 0..9 {
                intersections[i] = sudoku[floor_number + i * 3]
                    | sudoku[floor_number + i * 3 + 1]
                    | sudoku[floor_number + i * 3 + 2];
                only[i] = intersections[i] * (intersections[i].count_ones() <= 3) as u16;
            }
            let (resultant_mask, only) = generate_masks_from_intersections(intersections, only);

            let mut temp_total = 0;
            for (i, (row, only_row)) in resultant_mask.iter().zip(only.iter()).enumerate() {
                temp_total |= row;
                let row =
                    row & [consts::SUDOKU_MAX, *only_row][(only_row.count_ones() == 3) as usize];
                sudoku[floor_number + i * 3] &= row;
                sudoku[floor_number + i * 3 + 1] &= row;
                sudoku[floor_number + i * 3 + 2] &= row;

                sudoku_check *= (only_row.count_ones() <= 3) as u16;
                // If more than 3 digits can only be in intersection, then there is no solution
            }
            sudoku_check &= temp_total;
        }
        if sudoku_check != consts::SUDOKU_MAX {
            return false;
        }
        for tower_number in (0..3).map(|x| x * 3) {
            let mut only = [0; 9];
            let mut intersections = [0_u16; 9]; // Intersection
            for column in 0..3 {
                for layer in 0..3 {
                    let i = column * 3 + layer;
                    intersections[i] = sudoku[tower_number + layer * 27 + column]
                        | sudoku[tower_number + layer * 27 + column + 9]
                        | sudoku[tower_number + layer * 27 + column + 18];
                    only[i] = intersections[i] * (intersections[i].count_ones() <= 3) as u16;
                }
            }
            let (resultant_mask, only) = generate_masks_from_intersections(intersections, only);

            let mut temp_total = 0;

            for column_number in 0..3 {
                for layer in 0..3 {
                    let i = column_number * 3 + layer;
                    let column = resultant_mask[i];
                    let only_column = only[i];
                    temp_total |= column;
                    let column = column
                        & [consts::SUDOKU_MAX, only_column]
                            [(only_column.count_ones() == 3) as usize];
                    sudoku[tower_number + layer * 27 + column_number] &= column;
                    sudoku[tower_number + layer * 27 + column_number + 9] &= column;
                    sudoku[tower_number + layer * 27 + column_number + 18] &= column;

                    sudoku_check *= (only_column.count_ones() <= 3) as u16;
                    // If more than 3 digits can only be in intersection, then there is no solution
                }
            }
            sudoku_check &= temp_total;
        }
        self.cells = sudoku;
        sudoku_check == consts::SUDOKU_MAX
    }

    /**
    Convert the sudoku into a [u8; 81] containing the numerical form of each solved square
    */
    pub fn to_array(&self) -> [u8; 81] {
        let mut array = [0; 81];
        let mut temp = self.solved_squares;
        while temp != 0 {
            let square = get_last_digit!(temp, usize);
            if square >= 81 {
                break;
            }
            array[square] = self.cells[square].trailing_zeros() as u8 + 1;
        }
        array
    }

    pub fn to_bytes(&self) -> [u8; 81] {
        let mut chars = [b'.'; 81];
        let mut temp = self.solved_squares;
        while temp != 0 {
            let square = get_last_digit!(temp, usize);
            if square >= 81 {
                break;
            }
            chars[square] = (b"123456789")[self.cells[square].trailing_zeros() as usize];
        }
        chars
    }

    pub fn to_pencilmark_bytes(&self) -> [u8; 1605] {
        const INNER_ROW_LENGTH: usize = ((3 * 3 + 2) + 1) * 3 + 6;
        const OUTER_ROW_LENGTH: usize = INNER_ROW_LENGTH * 4 + 1;
        const TOTAL_LENGTH: usize = OUTER_ROW_LENGTH * 9 + INNER_ROW_LENGTH * 2;
        const FORMAT_ROW: [u8; 85] = *b"---+---+---+  +---+---+---+  +---+---+---\n\n---+---+---+  +---+---+---+  +---+---+---\n";
        let mut output_grid = [b'!'; TOTAL_LENGTH]; // '!' makes it easier to spot mistakes
        for row in 0..9 {
            let row_start = row * OUTER_ROW_LENGTH + row / 3 * (INNER_ROW_LENGTH);
            for inner_row in 0..3 {
                let inner_row_start = row_start + inner_row * INNER_ROW_LENGTH;
                let masks = (
                    1 << (inner_row * 3),
                    1 << (inner_row * 3 + 1),
                    1 << (inner_row * 3 + 2),
                );
                let output_digits = [
                    (b" 1", b" 2", b" 3"),
                    (b" 4", b" 5", b" 6"),
                    (b" 7", b" 8", b" 9"),
                ][inner_row];
                for column in 0..9 {
                    let digits = self.cells[row * 9 + column];
                    let index = inner_row_start + column * 4 + column / 3 * 3;
                    output_grid[index] = (output_digits.0)[(digits & masks.0 != 0) as usize];
                    output_grid[index + 1] = (output_digits.1)[(digits & masks.1 != 0) as usize];
                    output_grid[index + 2] = (output_digits.2)[(digits & masks.2 != 0) as usize];
                    output_grid[index + 3] = b'|';
                    output_grid[index + 4] = b' ';
                    output_grid[index + 5] = b' ';
                    output_grid[index + 6] = b'|';
                }

                output_grid[inner_row_start + INNER_ROW_LENGTH - 1] = b'\n';
            }
            for (ptr, value) in output_grid[row_start + INNER_ROW_LENGTH * 3..]
                .iter_mut()
                .zip(FORMAT_ROW.iter())
            {
                *ptr = *value;
            }
        }
        output_grid
    }
    /**
    Returns an iterator over all solutions
    */
    #[inline]
    pub fn iter(self) -> solution_iterator::QuickSolutionIterator {
        solution_iterator::QuickSolutionIterator::new(self)
    }
    /**
    Get the first solution.
    */
    #[inline]
    pub fn solve_one(self) -> Option<Self> {
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

    #[inline]
    pub fn has_solution(self) -> bool {
        self.count_solutions(1) == 1
    }

    /**
    Returns an empty sudoku grid, alternative to Sudoku::from([0; 81]) or Sudoku::from(vec![])
    */
    #[inline]
    pub const fn empty() -> Self {
        Self {
            cells: [consts::SUDOKU_MAX; 81],
            solved_squares: 0,
        }
    }

    /**
    Estimates the difficulty
    */
    #[inline]
    pub fn difficulty(self, count_steps: bool) -> Option<i32> {
        let mut difficulty = -(self.solved_cell_count() as i32);
        if count_steps {
            let mut iter = solution_iterator::SolutionIterator::<DifficultyRecording>::new(
                Self::from(self.to_array()),
            );
            if iter.next().is_none() || iter.next().is_some() {
                return None;
            }
            difficulty += iter.get_recording() as i32;
        }
        Some(difficulty)
    }

    #[cfg(feature = "alloc")]
    pub fn list_techniques(self) -> Vec<(String, Sudoku)> {
        let mut iter = solution_iterator::SolutionIterator::<FullRecording>::new(self);
        iter.next();
        iter.next();
        iter.get_recording()
    }

    pub fn solved_cell_count(&self) -> usize {
        (self.solved_squares & consts::SOLVED_SUDOKU).count_ones() as usize
    }
    #[cfg(feature = "generate")]
    pub fn generate<T>(rng: T, count_steps: bool) -> gen::SudokuGenerator<T>
    where
        T: rand::Rng + rand_core::RngCore,
    {
        gen::SudokuGenerator::new(rng, count_steps)
    }
    #[cfg(feature = "generate")]
    pub fn generate_from_seed<T>(
        self,
        rng: &mut T,
        cells_to_remove: usize,
        count_steps: bool,
    ) -> (Self, i32)
    where
        T: rand::Rng + rand_core::RngCore,
    {
        gen::generate_from_seed(self, rng, cells_to_remove, count_steps)
    }

    fn import<T: Iterator<Item = Option<u32>>>(square_iterator: T) -> Self {
        let mut sudoku = Self::empty();
        for (i, int) in square_iterator
            .enumerate()
            .take(81)
            .filter_map(|(i, item)| {
                item.filter(|x| *x <= 9)
                    .and_then(|x| x.checked_sub(1))
                    .map(|x| (i, x))
            })
        {
            sudoku.cells[i] = 1 << int;
            sudoku.solved_squares |= 1 << i;
        }
        sudoku
    }
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.solved_squares == other.solved_squares && self.cells[..] == other.cells[..]
    }
}

impl<T: TryInto<u32> + Copy> From<&[T]> for Sudoku {
    fn from(sudoku_array: &[T]) -> Self {
        Self::import(sudoku_array.iter().map(|x| (*x).try_into().ok()))
    }
}
impl<T: TryInto<u32> + Copy> From<&[T; 81]> for Sudoku {
    #[inline]
    fn from(sudoku_array: &[T; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<u32> + Copy> From<[T; 81]> for Sudoku {
    #[inline]
    fn from(sudoku_array: [T; 81]) -> Self {
        Self::from(&sudoku_array[..])
    }
}

#[cfg(feature = "alloc")]
impl<T: TryInto<u32> + Copy> From<Vec<T>> for Sudoku {
    #[inline]
    fn from(sudoku_array: Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}

#[cfg(feature = "alloc")]
impl<T: TryInto<u32> + Copy> From<&Vec<T>> for Sudoku {
    #[inline]
    fn from(sudoku_array: &Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}

impl FromStr for Sudoku {
    type Err = core::convert::Infallible;
    fn from_str(sudoku_str: &str) -> Result<Self, Self::Err> {
        Ok(Self::import(
            sudoku_str.chars().map(|character| character.to_digit(10)),
        ))
    }
}
