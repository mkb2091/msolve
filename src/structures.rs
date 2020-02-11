use crate::consts;

#[derive(Clone, Copy)]
pub struct SudokuUnsolvedSquares {
    squares: [u8; 81],
    end: usize,
    pos: usize,
    marked: usize,
}

impl Default for SudokuUnsolvedSquares {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for SudokuUnsolvedSquares {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.end {
            self.pos += 1;
            Some(self.squares[self.pos - 1])
        } else {
            None
        }
    }
}

impl SudokuUnsolvedSquares {
    pub fn new() -> SudokuUnsolvedSquares {
        SudokuUnsolvedSquares {
            squares: [
                77, 58, 21, 35, 25, 80, 61, 4, 76, 9, 54, 41, 63, 30, 13, 55, 20, 24, 27, 64, 0,
                72, 40, 69, 29, 16, 56, 68, 18, 39, 74, 43, 66, 70, 7, 33, 52, 73, 11, 47, 38, 34,
                26, 17, 31, 79, 59, 37, 50, 3, 48, 12, 67, 19, 14, 57, 1, 53, 36, 2, 51, 78, 28, 6,
                10, 22, 46, 60, 8, 32, 71, 15, 5, 42, 45, 49, 62, 44, 65, 75, 23,
            ],
            end: 81,
            pos: 0,
            marked: 0,
        }
    }
    pub fn mark_for_removal(&mut self) {
        self.marked = self.pos - 1;
    }
    pub fn remove_marked(&mut self) {
        self.end -= 1;
        assert!(self.marked <= self.end);
        self.squares.swap(self.marked, self.end);
    }
    pub fn remove(&mut self) {
        self.end -= 1;
        self.pos -= 1;
        assert!(self.pos <= self.end);
        self.squares.swap(self.pos, self.end);
    }
    pub fn start_iteration(&mut self) {
        self.pos = 0;
    }
}

#[derive(Clone, Copy)]
pub struct Sudoku {
    pub options: [u16; 81],
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku { options: [0; 81] }
    }
    pub fn from_array(sudoku: &[u8; 81]) -> Sudoku {
        let mut options: [u16; 81] = [consts::SUDOKU_MAX; 81];
        for (i, item) in sudoku.iter().enumerate() {
            if *item != 0 {
                options[i] = consts::SUDOKU_VALUES[*item as usize - 1];
            }
        }
        Sudoku { options }
    }

    pub fn to_array(&self) -> [u8; 81] {
        let mut array: [u8; 81] = [0; 81];
        for (square, processed) in self
            .options
            .iter()
            .enumerate()
            .map(|(square, &value)| (square, value & consts::SUDOKU_MAX))
        {
            if let Some((i, _)) = consts::SUDOKU_VALUES
                .iter()
                .enumerate()
                .find(|(_, &v)| processed == v)
            {
                array[square] = i as u8 + 1;
            }
        }
        array
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}
