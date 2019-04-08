use msolve;

fn no_errors(sudoku: [u8; 81], solution: [u8; 81]) -> Result<usize, bool> {
    let mut solver = msolve::MSolve::new();
    solver.set_sudoku(sudoku);
    solver.apply_techniques();
    let result = solver.to_array();
    let mut correct_count = 0;
    for i in 0..81 {
        if result[i] == 0 {
            if sudoku[i] != 0 {
                return Err(false);
            } else if 2_u16.pow((solution[i] - 1).into()) & solver.options[i] == 0 {
                return Err(false);
            }
        } else if result[i] != solution[i] {
            return Err(false);
        } else {
            correct_count += 1;
        }
    }
    Ok(correct_count)
}

fn full_match(sudoku: [u8; 81], solution: [u8; 81]) -> bool {
    let mut solver = msolve::MSolve::new();
    solver.set_sudoku(sudoku);
    solver.apply_techniques();
    let result = solver.to_array();
    for i in 0..81 {
        if result[i] != solution[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worlds_hardest_test() {
        let sudoku: [u8; 81] = [
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 7, 0, 0, 9, 0, 2, 0, 0, 0, 5,
            0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 4, 5, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0,
            0, 0, 0, 6, 8, 0, 0, 8, 5, 0, 0, 0, 1, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0,
        ];
        let solution: [u8; 81] = [
            8, 1, 2, 7, 5, 3, 6, 4, 9, 9, 4, 3, 6, 8, 2, 1, 7, 5, 6, 7, 5, 4, 9, 1, 2, 8, 3, 1, 5,
            4, 2, 3, 7, 8, 9, 6, 3, 6, 9, 8, 4, 5, 7, 2, 1, 2, 8, 7, 1, 6, 9, 5, 3, 4, 5, 2, 1, 9,
            7, 4, 3, 6, 8, 4, 3, 8, 5, 2, 6, 9, 1, 7, 7, 9, 6, 3, 1, 8, 4, 5, 2,
        ];
        let result = no_errors(sudoku, solution);
        assert!(result.is_ok());
        println!("worlds_hardest_test: {}", result.unwrap());
        assert!(full_match(sudoku, solution));
    }

    #[test]
    fn hardbrute_test() {
        let sudoku: [u8; 81] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0,
            0, 5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
            0, 0, 0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 9,
        ];
        let solution: [u8; 81] = [
            9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 4, 6, 1, 7, 3, 9, 8, 5, 3, 5, 1, 9, 2, 8, 7, 4, 6, 1, 2,
            8, 5, 3, 7, 6, 9, 4, 6, 3, 4, 8, 9, 2, 1, 5, 7, 7, 9, 5, 4, 6, 1, 8, 3, 2, 5, 1, 9, 2,
            8, 6, 4, 7, 3, 4, 7, 2, 3, 1, 9, 5, 6, 8, 8, 6, 3, 7, 4, 5, 2, 1, 9,
        ];
        let result = no_errors(sudoku, solution);
        assert!(result.is_ok());
        println!("hardbrute_test: {}", result.unwrap());
        assert!(full_match(sudoku, solution));
    }
    #[test]
    fn random17_test() {
        let sudoku: [u8; 81] = [
            0, 0, 0, 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 2, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 8, 0, 0, 0, 0,
            8, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0,
        ];
        let solution: [u8; 81] = [
            2, 6, 4, 7, 1, 5, 8, 3, 9, 1, 3, 7, 8, 9, 2, 6, 4, 5, 5, 9, 8, 4, 3, 6, 2, 7, 1, 4, 2,
            3, 1, 7, 8, 5, 9, 6, 8, 1, 6, 5, 4, 9, 7, 2, 3, 7, 5, 9, 6, 2, 3, 4, 1, 8, 3, 7, 5, 2,
            8, 1, 9, 6, 4, 9, 8, 2, 3, 6, 4, 1, 5, 7, 6, 4, 1, 9, 5, 7, 3, 8, 2,
        ];
        let result = no_errors(sudoku, solution);
        assert!(result.is_ok());
        println!("random17_test: {}", result.unwrap());
        assert!(full_match(sudoku, solution));
    }
    #[test]
    fn easy_8802_test() {
        let sudoku: [u8; 81] = [
            0, 5, 0, 4, 0, 0, 9, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 5, 9, 0, 0, 0, 0, 7, 6, 3, 0, 0, 7,
            5, 0, 0, 0, 0, 0, 4, 4, 1, 0, 0, 0, 0, 7, 9, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 9,
            0, 0, 2, 7, 1, 7, 0, 0, 0, 0, 5, 4, 0, 6, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        ];
        let solution: [u8; 81] = [
            6, 5, 3, 4, 8, 2, 9, 1, 7, 1, 2, 7, 6, 9, 3, 8, 4, 5, 9, 8, 4, 5, 1, 7, 6, 3, 2, 2, 7,
            5, 8, 3, 9, 1, 6, 4, 4, 1, 8, 2, 5, 6, 7, 9, 3, 3, 6, 9, 1, 7, 4, 5, 2, 8, 5, 3, 6, 9,
            4, 8, 2, 7, 1, 7, 9, 1, 3, 2, 5, 4, 8, 6, 8, 4, 2, 7, 6, 1, 3, 5, 9,
        ];
        let result = no_errors(sudoku, solution);
        assert!(result.is_ok());
        println!("easy_8802_test: {}", result.unwrap());
        assert!(full_match(sudoku, solution));
    }
}
