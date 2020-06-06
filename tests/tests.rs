#[cfg(test)]
mod tests {
    #[test]
    fn top2365() {
        use std::io::BufRead;
        let solver = msolve::Solver::new();
        let file_in = std::fs::File::open("bench_sudokus/top2365").expect("Failed to open file");
        let mut buf = std::io::BufReader::new(file_in);
        let mut line = String::with_capacity(81);
        while buf.read_line(&mut line).unwrap() > 0 {
            if let Ok(sudoku) = sudoku::Sudoku::from_str_line(&line) {
                if let Some(solution) = sudoku.solve_unique() {
                    assert_eq!(
                        solution.to_bytes().to_vec(),
                        solver.solve_string(&line).unwrap().to_vec()
                    );
                } else if let Some(solution) = solver.solve_string(&line) {
                    println!("Sudoku: {}", sudoku);
                    println!(
                        "My Solution: {}",
                        solution
                            .iter()
                            .map(|value| value.to_string())
                            .collect::<String>()
                    );
                } else {
                    println!("Neither could solve sudoku: {}", sudoku);
                }
            }
            line.clear();
        }
    }
    #[test]
    fn sudoku17_list() {
        use std::io::BufRead;
        let solver = msolve::Solver::new();
        let file_in = std::fs::File::open("bench_sudokus/sudoku17").expect("Failed to open file");
        let mut buf = std::io::BufReader::new(file_in);
        let mut line = String::with_capacity(81);
        while buf.read_line(&mut line).unwrap() > 0 {
            if let Ok(sudoku) = sudoku::Sudoku::from_str_line(&line) {
                if let Some(solution) = sudoku.solve_unique() {
                    assert_eq!(
                        solution.to_bytes().to_vec(),
                        solver.solve_string(&line).unwrap().to_vec()
                    );
                } else if let Some(solution) = solver.solve_string(&line) {
                    println!("Sudoku: {}", sudoku);
                    println!(
                        "My Solution: {}",
                        solution
                            .iter()
                            .map(|value| value.to_string())
                            .collect::<String>()
                    );
                } else {
                    println!("Neither could solve sudoku: {}", sudoku);
                }
            }
            line.clear();
        }
    }
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
        let solver = msolve::Solver::new();
        assert_eq!(
            solution.to_vec(),
            solver.solve_array(&sudoku).unwrap().to_vec()
        );
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
        let solver = msolve::Solver::new();
        assert_eq!(
            solution.to_vec(),
            solver.solve_array(&sudoku).unwrap().to_vec()
        );
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
        let solver = msolve::Solver::new();
        assert_eq!(
            solution.to_vec(),
            solver.solve_array(&sudoku).unwrap().to_vec()
        );
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
        let solver = msolve::Solver::new();
        assert_eq!(
            solution.to_vec(),
            solver.solve_array(&sudoku).unwrap().to_vec()
        );
    }
}
