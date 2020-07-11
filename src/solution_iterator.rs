use crate::{consts, get_last_digit, Sudoku};

#[cfg(feature = "smallvec")]
type SudokuBackTrackingVec = smallvec::SmallVec<[Sudoku; 10]>;
#[cfg(not(feature = "smallvec"))]
type SudokuBackTrackingVec = Vec<Sudoku>;

/**

Trait to iterate through solutions

Additional requirement to implement

impl Iterator for {{Name}} {
    type Item = Sudoku;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_solution()
    }
}

*/
trait SolutionIterator: Iterator {
    fn prepare_sudoku(mut sudoku: Sudoku) -> Option<Sudoku> {
        let mut temp = sudoku.solved_squares;
        let mut valid = true;
        while temp != 0 {
            let square = get_last_digit!(temp, usize);
            if sudoku.cells[square].is_power_of_two() {
                sudoku.apply_number(square);
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            Some(sudoku)
        } else {
            None
        }
    }
    fn pop_next_state(&mut self) -> Option<Sudoku>;
    fn add_state(&mut self, state: Sudoku);
    /**
    Get the next solution
    Perform a single iteration solving
    Call hidden_singles for each unsolved cell, and call apply_number for each newly solved cell\
    Select unsolved cell with least possible values
    For each possible value, clone the sudoku state, set the cell to the value and add to the state list
    */
    fn get_next_solution(&mut self) -> Option<Sudoku> {
        'outer: while let Some(mut state) = self.pop_next_state() {
            self.record_step(&state);
            if state.solved_squares.count_ones() == 81 {
                return Some(state);
            }
            let mut min: (usize, u32) = (0, std::u32::MAX);
            let mut temp = !state.solved_squares;
            loop {
                let square = get_last_digit!(temp, usize);
                if square >= 81 {
                    break;
                }
                if state.cells[square] == 0 {
                    continue 'outer;
                }
                if state.cells[square].is_power_of_two()
                    || match state.hidden_singles(square).ok() {
                        Some(result) => {
                            if result {
                                self.record_hidden_single(square, &state);
                            };
                            result
                        }
                        None => continue 'outer,
                    }
                {
                    self.record_apply_number(square, &state);
                    if state.solved_squares.count_ones() == 80 {
                        state.solved_squares |= 1 << square;
                        return Some(state);
                    }
                    state.apply_number(square);
                } else {
                    let possible_values = state.cells[square].count_ones();
                    if possible_values < min.1 {
                        min = (square, possible_values);
                    }
                }
            }
            debug_assert!(min.1 <= 9);
            if state.solved_squares.count_ones() >= consts::SCANNING_CUTOFF || {
                self.record_scan(&state);
                state.scan()
            } {
                let mut value = state.cells[min.0];
                while value != 0 {
                    let i = get_last_digit!(value, u16);
                    let mut new = state;
                    new.cells[min.0] = 1 << i;
                    new.apply_number(min.0);
                    self.add_state(new);
                }
            }
        }
        None
    }
    fn record_step(&mut self, _: &Sudoku) {}
    fn record_apply_number(&mut self, _: usize, _: &Sudoku) {}
    fn record_scan(&mut self, _: &Sudoku) {}
    fn record_hidden_single(&mut self, _: usize, _: &Sudoku) {}
}

pub struct QuickSolutionIterator {
    routes: SudokuBackTrackingVec,
}

impl QuickSolutionIterator {
    pub fn new(sudoku: Sudoku) -> Self {
        let sudoku = Self::prepare_sudoku(sudoku);
        let mut routes = SudokuBackTrackingVec::with_capacity(10);
        if let Some(sudoku) = sudoku {
            routes.push(sudoku)
        }
        Self { routes }
    }
}

impl SolutionIterator for QuickSolutionIterator {
    fn pop_next_state(&mut self) -> Option<Sudoku> {
        self.routes.pop()
    }
    fn add_state(&mut self, state: Sudoku) {
        self.routes.push(state);
    }
}

impl Iterator for QuickSolutionIterator {
    type Item = Sudoku;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_solution()
    }
}

pub(crate) struct DifficultySolutionIterator {
    routes: SudokuBackTrackingVec,
    step_count: usize,
    apply_number_count: usize,
    scan_count: usize,
    hidden_single_count: usize,
}

impl DifficultySolutionIterator {
    pub fn new(sudoku: Sudoku) -> Self {
        let sudoku = Self::prepare_sudoku(sudoku);
        let mut routes = SudokuBackTrackingVec::with_capacity(10);
        if let Some(sudoku) = sudoku {
            routes.push(sudoku)
        }
        Self {
            routes,
            step_count: 0,
            apply_number_count: 0,
            scan_count: 0,
            hidden_single_count: 0,
        }
    }
    pub fn get_difficulty(&self) -> usize {
        self.step_count + self.apply_number_count + self.scan_count + self.hidden_single_count
    }
}

impl SolutionIterator for DifficultySolutionIterator {
    fn pop_next_state(&mut self) -> Option<Sudoku> {
        self.routes.pop()
    }
    fn add_state(&mut self, state: Sudoku) {
        self.routes.push(state);
    }
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
}

impl Iterator for DifficultySolutionIterator {
    type Item = Sudoku;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_solution()
    }
}

pub(crate) struct TechniqueRecording {
    routes: SudokuBackTrackingVec,
    techniques: Vec<(String, Sudoku)>,
}

impl TechniqueRecording {
    pub fn new(sudoku: Sudoku) -> Self {
        let sudoku = Self::prepare_sudoku(sudoku);
        let mut routes = SudokuBackTrackingVec::with_capacity(10);
        if let Some(sudoku) = sudoku {
            routes.push(sudoku)
        }
        Self {
            routes,
            techniques: Vec::new(),
        }
    }
    pub fn get_techniques_used(&self) -> Vec<(String, Sudoku)> {
        let mut result = self.techniques.clone();
        result.dedup_by_key(|(_, sudoku)| *sudoku);
        result
    }
}

impl SolutionIterator for TechniqueRecording {
    fn pop_next_state(&mut self) -> Option<Sudoku> {
        self.routes.pop()
    }
    fn add_state(&mut self, state: Sudoku) {
        self.routes.push(state);
    }
    fn record_step(&mut self, _: &Sudoku) {}
    fn record_apply_number(&mut self, square: usize, state: &Sudoku) {
        self.techniques.push((
            format!(
                "Found naked single: R{}C{}",
                (square / 9) + 1,
                (square % 9) + 1
            ),
            *state,
        ))
    }
    fn record_scan(&mut self, state: &Sudoku) {
        self.techniques.push(("Scanned".to_string(), *state))
    }
    fn record_hidden_single(&mut self, square: usize, state: &Sudoku) {
        self.techniques.push((
            format!(
                "Found hidden single: R{}C{}",
                (square / 9) + 1,
                (square % 9) + 1
            ),
            *state,
        ))
    }
}

impl Iterator for TechniqueRecording {
    type Item = Sudoku;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_solution()
    }
}
