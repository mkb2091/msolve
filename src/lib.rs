#![no_std]

mod consts;
pub mod techniques;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[derive(Clone, Copy)]
pub struct MSolve {
    pub options: [u16; 81],
    to_explore: [u8; 81],
    pos: usize,
}

impl MSolve {
    pub fn new() -> MSolve {
        MSolve {
            options: [0; 81],
            to_explore: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
                65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
            ],
            pos: 0,
        }
    }
    pub fn set_sudoku(&mut self, sudoku: [u8; 81]) {
        self.options = [consts::SUDOKU_MAX; 81];
        self.pos = 81;
        self.to_explore = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        ];
        let mut pos = 0;
        for (d, (i, s)) in self
            .options
            .iter_mut()
            .zip(sudoku.iter().enumerate())
            .filter(|(_, (_, &s))| s != 0)
        {
            *d = consts::SUDOKU_VALUES[(*s - 1) as usize];
            self.to_explore[i] = self.to_explore[pos];
            self.to_explore[pos] = i as u8;
            pos += 1;
        }
    }
    pub fn apply_techniques(&mut self) -> bool {
        let mut changed = true;
        while changed {
            changed = false;
            let mut x = 0;
            while x < self.pos {
                let square = self.to_explore[x] as usize;
                if !techniques::hidden_singles(&mut self.options, square) {
                    return false;
                }
                changed |= match consts::OPTION_COUNT_CACHE[self.options[square] as usize] {
                    0 => return false,
                    1 => techniques::apply_number(&mut self.options, square),
                    2 => techniques::naked_pair(&mut self.options, square),
                    3 => techniques::naked_triple(&mut self.options, square),
                    _ => false,
                };
                if self.options[square] >= consts::SQUARE_DONE {
                    self.pos -= 1;
                    if self.pos != x {
                        self.to_explore[x] = self.to_explore[self.pos];
                        self.to_explore[self.pos] = square as u8;
                    }
                } else {
                    x += 1;
                }
            }
        }
        true
    }
    pub fn next(&mut self) {}
    pub fn to_array(&self) -> [u8; 81] {
        let mut array: [u8; 81] = [0; 81];
        for (square, processed) in self
            .options
            .iter()
            .enumerate()
            .map(|(square, &value)| (square, value & consts::SUDOKU_MAX))
            .filter(|(_, processed)| consts::OPTION_COUNT_CACHE[*processed as usize] == 1)
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

impl Default for MSolve {
    fn default() -> Self {
        Self::new()
    }
}
