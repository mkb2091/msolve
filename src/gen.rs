use crate::*;

type SudokuScore = i32;

use rand::distributions::Distribution;

pub fn minimise(
    sudoku: Sudoku,
    old_score: Option<SudokuScore>,
    count_steps: bool,
) -> Option<(Sudoku, SudokuScore)> {
    let mut old: Option<(Sudoku, SudokuScore)> = old_score
        .or_else(|| sudoku.difficulty(count_steps))
        .map(|old_score| (sudoku, old_score));
    let mut changed = false;
    let mut removable: u128 = u128::MAX;
    while let Some((old_sudoku, old_score)) = old {
        let mut best_score = old_score;
        let mut best_sudoku: Option<Sudoku> = None;
        let mut temp = old_sudoku.solved_squares & consts::SOLVED_SUDOKU & removable;
        let mut array = old_sudoku.to_array();
        while temp != 0 {
            let square = get_last_digit!(temp, usize);
            let old_value = array[square];
            debug_assert_ne!(old_value, 0);
            array[square] = 0;
            let new = Sudoku::from(&array);
            if let Some(new_score) = new.difficulty(count_steps) {
                if new_score > best_score && (count_steps || new.has_single_solution()) {
                    best_score = new_score;
                    best_sudoku = Some(new);
                    changed = true;
                    if !count_steps {
                        array[square] = old_value;
                        break;
                    }
                }
            } else {
                removable -= 1 << square;
            }
            array[square] = old_value;
        }
        if best_sudoku.is_none() && changed {
            debug_assert!(old.is_some());
            return old;
        }
        old = best_sudoku.map(|best_sudoku| (best_sudoku, best_score));
    }
    None
}

fn mutate(
    sudoku: Sudoku,
    old_score: Option<SudokuScore>,
    count_steps: bool,
) -> Option<(Sudoku, SudokuScore)> {
    let old_score = old_score.or_else(|| sudoku.difficulty(count_steps));
    if !(count_steps || sudoku.has_single_solution()) {
        return None;
    }
    let mut best: Option<(Sudoku, SudokuScore)> = None;
    let mut temp = sudoku.solved_squares & consts::SOLVED_SUDOKU;
    let mut array = sudoku.to_array();
    while temp != 0 {
        let s1 = get_last_digit!(temp, usize);
        let old = array[s1];
        array[s1] = 0;
        let mut temp2 = !sudoku.solved_squares & consts::SOLVED_SUDOKU;
        while temp2 != 0 {
            let s2 = get_last_digit!(temp2, usize);
            let mut temp3 = sudoku.cells[s2];
            while temp3 != 0 {
                let value = get_last_digit!(temp3, u8);
                array[s2] = value;
                let sudoku = Sudoku::from(&array);
                if sudoku.has_single_solution() {
                    best = minimise(
                        sudoku,
                        best.map(|(_, score)| score).or(old_score),
                        count_steps,
                    )
                    .or(best)
                };
            }
            array[s2] = 0;
        }
        array[s1] = old;
    }
    best
}

fn generate<T>(rng: &mut T, count_steps: bool) -> (Sudoku, SudokuScore)
where
    T: rand::Rng + rand_core::RngCore,
{
    let mut sudoku = Sudoku::empty();
    let cell_distribution = rand::distributions::Uniform::new(0, 81);
    while (sudoku.solved_squares & consts::SOLVED_SUDOKU) != consts::SOLVED_SUDOKU {
        let index = cell_distribution.sample(rng);
        if sudoku.solved_squares & (1 << index) != 0 {
            continue;
        }
        let mut temp = sudoku;
        let mut value = temp.cells[index];
        debug_assert_ne!(value, 0);
        let chosen_value_index = rng.gen_range(0, value.count_ones());
        let mut i = get_last_digit!(value, usize);
        for _ in 0..chosen_value_index {
            i = get_last_digit!(value, usize);
        }
        temp.cells[index] = 1 << i;
        temp.apply_number(index);
        temp.scan();
        match temp.count_solutions(2) {
            2 => sudoku = temp,
            1 => {
                sudoku = temp;
                break;
            }
            0 => sudoku.cells[index] -= 1 << i,
            _ => {
                debug_assert!(false, "More than 2 returned from count_solutions(2)");
            }
        }
    }
    minimise(sudoku, None, count_steps).unwrap_or_else(|| {
        (
            sudoku,
            sudoku.difficulty(count_steps).unwrap_or_else(|| {
                debug_assert!(false);
                i32::MIN
            }),
        )
    })
}

pub fn generate_from_seed<T>(
    sudoku: Sudoku,
    rng: &mut T,
    cells_to_remove: usize,
    count_steps: bool,
) -> (Sudoku, SudokuScore)
where
    T: rand::Rng + rand_core::RngCore,
{
    let mut array = sudoku.to_array();
    let mut solved_squares = sudoku.solved_squares & consts::SOLVED_SUDOKU;
    let desired_solved_count = solved_squares
        .count_ones()
        .saturating_sub(cells_to_remove as u32);
    while solved_squares.count_ones() > desired_solved_count || !Sudoku::from(array).has_solution()
    {
        let solved_index = rng.gen_range(0, solved_squares.count_ones() as usize);
        let mut temp = solved_squares;
        let mut i = get_last_digit!(temp, usize);
        for _ in 0..solved_index {
            i = get_last_digit!(temp, usize);
        }
        debug_assert_ne!(array[i], 0);
        array[i] = 0;

        solved_squares -= 1 << i;
    }

    let mut sudoku = Sudoku::from(array);
    sudoku.scan();
    let cell_distribution = rand::distributions::Uniform::new(0, 81);
    while (sudoku.solved_squares & consts::SOLVED_SUDOKU) != consts::SOLVED_SUDOKU {
        let index = cell_distribution.sample(rng);
        if sudoku.solved_squares & (1 << index) != 0 {
            continue;
        }
        let mut temp = sudoku;
        let mut value = temp.cells[index];
        debug_assert_ne!(value, 0);
        let chosen_value_index = rng.gen_range(0, value.count_ones());
        let mut i = get_last_digit!(value, usize);
        for _ in 0..chosen_value_index {
            i = get_last_digit!(value, usize);
        }
        temp.cells[index] = 1 << i;
        temp.apply_number(index);
        temp.scan();
        match temp.count_solutions(2) {
            2 => sudoku = temp,
            1 => {
                sudoku = temp;
                break;
            }
            0 => sudoku.cells[index] -= 1 << i,
            _ => {
                debug_assert!(false, "More than 2 returned from count_solutions(2)");
            }
        }
    }
    minimise(sudoku, None, count_steps).unwrap_or_else(|| {
        (
            sudoku,
            sudoku.difficulty(count_steps).unwrap_or_else(|| {
                debug_assert!(false);
                i32::MIN
            }),
        )
    })
}

pub struct SudokuGenerator<T>
where
    T: rand::Rng + rand_core::RngCore,
{
    rng: T,
    current: Option<(Sudoku, SudokuScore)>,
    count_steps: bool,
}

impl<T> SudokuGenerator<T>
where
    T: rand::Rng + rand_core::RngCore,
{
    pub fn new(rng: T, count_steps: bool) -> Self
    where
        T: rand::Rng + rand_core::RngCore,
    {
        SudokuGenerator {
            rng,
            current: None,
            count_steps,
        }
    }
}

impl<T> Iterator for SudokuGenerator<T>
where
    T: rand::Rng + rand_core::RngCore,
{
    type Item = (Sudoku, SudokuScore);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = mutate(current.0, Some(current.1), self.count_steps);
        }
        if self.current.is_none() {
            self.current = Some(generate(&mut self.rng, self.count_steps));
        }
        self.current
    }
}
