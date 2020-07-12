use crate::{consts, get_last_digit, Sudoku};

#[cfg(feature = "smallvec")]
type SudokuBackTrackingVec = smallvec::SmallVec<[Sudoku; 10]>;
#[cfg(not(feature = "smallvec"))]
type SudokuBackTrackingVec = Vec<Sudoku>;

pub trait TechniqueRecording: Default {
    fn record_step(&mut self, _: &Sudoku) {}
    fn record_apply_number(&mut self, _: usize, _: &Sudoku) {}
    fn record_scan(&mut self, _: &Sudoku) {}
    fn record_hidden_single(&mut self, _: usize, _: &Sudoku) {}
    type Output;
    fn get_recording(&self) -> Self::Output;
}

pub struct SolutionIterator<T: TechniqueRecording> {
    routes: SudokuBackTrackingVec,
    recording: T,
}

impl<T> SolutionIterator<T>
where
    T: TechniqueRecording,
{
    pub fn new(mut sudoku: Sudoku) -> Self {
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
        let mut routes = SudokuBackTrackingVec::with_capacity(10);
        if valid {
            routes.push(sudoku);
        }
        Self {
            routes,
            recording: T::default(),
        }
    }
    pub fn get_recording(&self) -> T::Output {
        self.recording.get_recording()
    }
}

impl<T> Iterator for SolutionIterator<T>
where
    T: TechniqueRecording,
{
    type Item = Sudoku;
    fn next(&mut self) -> Option<Self::Item> {
        'outer: while let Some(mut state) = self.routes.pop() {
            self.recording.record_step(&state);
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
                                self.recording.record_hidden_single(square, &state);
                            };
                            result
                        }
                        None => continue 'outer,
                    }
                {
                    self.recording.record_apply_number(square, &state);
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
                self.recording.record_scan(&state);
                state.scan()
            } {
                let mut value = state.cells[min.0];
                while value != 0 {
                    let i = get_last_digit!(value, u16);
                    let mut new = state;
                    new.cells[min.0] = 1 << i;
                    self.recording.record_apply_number(min.0, &state);
                    new.apply_number(min.0);
                    self.routes.push(new);
                }
            }
        }
        None
    }
}

#[derive(Default)]
pub struct NoRecording {}

impl TechniqueRecording for NoRecording {
    type Output = ();
    fn get_recording(&self) {}
}

pub type QuickSolutionIterator = SolutionIterator<NoRecording>;
