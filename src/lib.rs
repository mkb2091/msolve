#![no_std]

mod consts;
mod techniques;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct MSolve {
    options: [u16; 81],
}

impl MSolve {
    pub fn new() -> MSolve {
        MSolve { options: [0; 81] }
    }
    pub fn set_sudoku(&mut self, sudoku: [u8; 81]) {
        self.options = [consts::SUDOKU_MAX; 81];
        for (d, s) in self.options.iter_mut().zip(sudoku.iter()) {
            if *s != 0 {
                *d = consts::SUDOKU_VALUES[(*s - 1) as usize] | consts::SUDOKU_TECHNIQUES_TOTAL;
            }
        }
    }
    pub fn apply_techniques(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            if techniques::naked_n(&mut self.options) {
                changed = true;
            }
            if techniques::hidden_singles(&mut self.options) {
                changed = true;
            }
        }
    }
    pub fn next(&mut self) {}
    pub fn to_array(&self) -> [u8; 81] {
        let mut array: [u8; 81] = [0; 81];
        for (square, value) in self.options.iter().enumerate() {
            let processed = value & consts::SUDOKU_VALUES_TOTAL;
            if consts::OPTION_COUNT_CACHE[processed as usize] == 1 {
                for (i, v) in consts::SUDOKU_VALUES.iter().enumerate() {
                    if processed == *v {
                        array[square] = i as u8 + 1;
                    }
                }
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
