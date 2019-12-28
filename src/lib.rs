mod consts;
pub mod structures;
pub mod techniques;

#[derive(Clone, Copy)]
pub struct MSolve {
    pub options: structures::Sudoku,
    to_explore: structures::SudokuUnsolvedSquares,
}

impl MSolve {
    pub fn new() -> MSolve {
        MSolve {
            options: structures::Sudoku::new(),
            to_explore: structures::SudokuUnsolvedSquares::new(),
        }
    }
    pub fn set_sudoku(&mut self, sudoku: &[u8; 81]) {
        self.options = structures::Sudoku::from_array(sudoku);
        self.to_explore = structures::SudokuUnsolvedSquares::new();
    }

    pub fn process(&mut self, routes: &mut Vec<MSolve>) -> bool {
        let mut values: Vec<u8> = Vec::with_capacity(9);
        loop {
            let mut min_options = 20;
            let mut min_square = 0;
            self.to_explore.start_iteration();
            while let Ok(square) = self.to_explore.next() {
                if !techniques::hidden_singles(&mut self.options, square as usize) {
                    return false;
                }
                let option_count =
                    consts::OPTION_COUNT_CACHE[self.options.options[square as usize] as usize];
                if option_count < min_options {
                    match option_count {
                        0 => return false,
                        1 => {
                            techniques::apply_number(&mut self.options, square as usize);
                            self.to_explore.remove();
                        }
                        _ => {
                            min_options = option_count;
                            min_square = square;
                            self.to_explore.mark_for_removal();
                        }
                    }
                }
            }
            if min_options != 20 {
                values.clear();
                let options = self.options.options[min_square as usize];
                for (i, item) in consts::SUDOKU_VALUES.iter().enumerate() {
                    if options & *item != 0 {
                        values.push(i as u8 + 1);
                    }
                }
                if values.is_empty() {
                    return false;
                }
                self.to_explore.remove_marked();
                let item = values.pop().unwrap();
                for value in values.iter() {
                    let mut clone = *self;
                    clone.options.options[min_square as usize] =
                        consts::SUDOKU_VALUES[*value as usize - 1];
                    techniques::apply_number(&mut clone.options, min_square as usize);
                    routes.push(clone);
                }
                self.options.options[min_square as usize] =
                    consts::SUDOKU_VALUES[item as usize - 1];
                techniques::apply_number(&mut self.options, min_square as usize);
            } else {
                return true;
            }
        }
    }
    pub fn next(&mut self) {}
    pub fn to_array(&self) -> [u8; 81] {
        let mut array: [u8; 81] = [0; 81];
        for (square, processed) in self
            .options
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

#[inline(never)]
pub fn solve(sudoku: &[u8; 81]) -> [u8; 81] {
    let mut solver = MSolve::new();
    solver.set_sudoku(&sudoku);
    let mut routes: Vec<MSolve> = vec![solver];
    routes.reserve(32);
    loop {
        if let Some(mut route) = routes.pop() {
            let result = route.process(&mut routes);
            if result {
                return route.to_array();
            }
        } else {
            break;
        }
    }
    panic!("Empty routes, but still unsolved");
}
