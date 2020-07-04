#[cfg(default)]
extern crate smallvec;

use std::convert::TryInto;

#[cfg(feature = "smallvec")]
type SudokuBackTrackingVec = smallvec::SmallVec<[Sudoku; 10]>;
#[cfg(not(feature = "smallvec"))]
type SudokuBackTrackingVec = Vec<Sudoku>;

/** Max 9 bit number */
const SUDOKU_MAX: u16 = (1 << 9) - 1;

macro_rules! get_last_digit {
    ($x:ident, $value_type:ty) => {{
        let value = $x.trailing_zeros();
        $x -= 1 << value;
        value as $value_type
    }};
}

const fn cells_in_house(square: usize) -> [u8; 20] {
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
    squares_to_change |= (1
        + (1 << 1)
        + (1 << 2)
        + (1 << 9)
        + (1 << 10)
        + (1 << 11)
        + (1 << 18)
        + (1 << 19)
        + (1 << 20))
        << box_start;
    squares_to_change &= !(1 << square);
    let mut squares_to_change_array = [0; 20];
    squares_to_change_array[0] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[1] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[2] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[3] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[4] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[5] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[6] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[7] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[8] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[9] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[10] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[11] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[12] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[13] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[14] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[15] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[16] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[17] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[18] = get_last_digit!(squares_to_change, u8);
    squares_to_change_array[19] = squares_to_change.trailing_zeros() as u8;

    //For when while in const is stablized
    /*let mut i = 0;
    while i < 20 {
        squares_to_change_array[i] = get_last_digit!(squares_to_change, u8);
        i += 1;
    }*/
    squares_to_change_array
}

const CELLS_TO_CHANGE: [[u8; 20]; 81] = {
    [
        cells_in_house(0),
        cells_in_house(1),
        cells_in_house(2),
        cells_in_house(3),
        cells_in_house(4),
        cells_in_house(5),
        cells_in_house(6),
        cells_in_house(7),
        cells_in_house(8),
        cells_in_house(9),
        cells_in_house(10),
        cells_in_house(11),
        cells_in_house(12),
        cells_in_house(13),
        cells_in_house(14),
        cells_in_house(15),
        cells_in_house(16),
        cells_in_house(17),
        cells_in_house(18),
        cells_in_house(19),
        cells_in_house(20),
        cells_in_house(21),
        cells_in_house(22),
        cells_in_house(23),
        cells_in_house(24),
        cells_in_house(25),
        cells_in_house(26),
        cells_in_house(27),
        cells_in_house(28),
        cells_in_house(29),
        cells_in_house(30),
        cells_in_house(31),
        cells_in_house(32),
        cells_in_house(33),
        cells_in_house(34),
        cells_in_house(35),
        cells_in_house(36),
        cells_in_house(37),
        cells_in_house(38),
        cells_in_house(39),
        cells_in_house(40),
        cells_in_house(41),
        cells_in_house(42),
        cells_in_house(43),
        cells_in_house(44),
        cells_in_house(45),
        cells_in_house(46),
        cells_in_house(47),
        cells_in_house(48),
        cells_in_house(49),
        cells_in_house(50),
        cells_in_house(51),
        cells_in_house(52),
        cells_in_house(53),
        cells_in_house(54),
        cells_in_house(55),
        cells_in_house(56),
        cells_in_house(57),
        cells_in_house(58),
        cells_in_house(59),
        cells_in_house(60),
        cells_in_house(61),
        cells_in_house(62),
        cells_in_house(63),
        cells_in_house(64),
        cells_in_house(65),
        cells_in_house(66),
        cells_in_house(67),
        cells_in_house(68),
        cells_in_house(69),
        cells_in_house(70),
        cells_in_house(71),
        cells_in_house(72),
        cells_in_house(73),
        cells_in_house(74),
        cells_in_house(75),
        cells_in_house(76),
        cells_in_house(77),
        cells_in_house(78),
        cells_in_house(79),
        cells_in_house(80),
    ]
    //For when while in const is stablized
    /*let mut data = [[0; 20]; 81];
    let mut i = 0;
    while i < 81 {
        data[i] = cells_in_house(i);
        i += 1;
    }
    data*/
};

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
            let square = get_last_digit!(temp, usize);
            if square >= 81 {
                break;
            }
            if self.cells[square] == 0 {
                return Err(());
            }
            if self.cells[square].is_power_of_two() || self.hidden_singles(square)? {
                if self.solved_squares.count_ones() == 80 {
                    self.solved_squares |= 1 << square;
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
                let i = get_last_digit!(value, u16);
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

    pub fn to_bytes(&self) -> [u8; 81] {
        let mut chars = [b'.'; 81];
        let mut temp = self.solved_squares;
        loop {
            let square = temp.trailing_zeros() as usize;
            if square >= 81 {
                break;
            }
            temp -= 1 << square;
            chars[square] = (b"123456789")[self.cells[square].trailing_zeros() as usize];
        }
        chars
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
            sudoku.apply_number(i);
        }
        sudoku.scan();
        sudoku
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
impl<T: TryInto<u32> + Copy> From<Vec<T>> for Sudoku {
    #[inline]
    fn from(sudoku_array: Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}
impl<T: TryInto<u32> + Copy> From<&Vec<T>> for Sudoku {
    #[inline]
    fn from(sudoku_array: &Vec<T>) -> Self {
        Self::from(&sudoku_array[..])
    }
}

impl From<&str> for Sudoku {
    fn from(sudoku_str: &str) -> Self {
        Self::import(sudoku_str.chars().map(|character| character.to_digit(10)))
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

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.to_bytes()).unwrap())
    }
}

impl std::fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const INNER_ROW_LENGTH: usize = ((3 * 3 + 2) + 1) * 3 + 6;
        const OUTER_ROW_LENGTH: usize = INNER_ROW_LENGTH * 4 + 1;
        const TOTAL_LENGTH: usize = OUTER_ROW_LENGTH * 9 + INNER_ROW_LENGTH * 2;
        const FORMAT_ROW: [u8; 85] = *b"---+---+---+  +---+---+---+  +---+---+---\n\n---+---+---+  +---+---+---+  +---+---+---\n";
        let mut output_grid = [b'!'; TOTAL_LENGTH];
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
        write!(
            f,
            "{}",
            std::str::from_utf8(&output_grid[..TOTAL_LENGTH - INNER_ROW_LENGTH - 1]).unwrap()
        )
    }
}
