#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    fn test_file(path: &str) {
        use std::io::BufRead;
        let file_in = std::fs::File::open(path).expect("Failed to open file");
        let mut buf = std::io::BufReader::new(file_in);
        let mut line = String::with_capacity(81);
        while buf.read_line(&mut line).unwrap() > 0 {
            if let Ok(sudoku) = sudoku::Sudoku::from_str_line(&line) {
                if let Some(solution) = sudoku.solve_unique() {
                    assert_eq!(
                        &solution.to_bytes()[..],
                        &line
                            .parse::<msolve::Sudoku>()
                            .unwrap()
                            .solve_unique()
                            .unwrap()
                            .to_array()[..]
                    );
                } else if let Ok(msolve_sudoku) = line.parse::<msolve::Sudoku>() {
                    assert!(msolve_sudoku.solve_unique().is_none());
                    assert_eq!(
                        sudoku.count_at_most(100),
                        msolve_sudoku.count_solutions(100)
                    );
                }
            }
            line.clear();
        }
    }
    #[test]
    fn top2365() {
        test_file("bench_sudokus/top2365");
    }
    #[test]
    fn sudoku17_list() {
        test_file("bench_sudokus/sudoku17");
    }
    #[test]
    fn kaggle_list() {
        test_file("bench_sudokus/kaggle.txt");
    }
    #[test]
    fn forum_hardest_1905_list() {
        test_file("bench_sudokus/forum_hardest_1905");
    }
    #[test]
    fn gen_puzzles_list() {
        test_file("bench_sudokus/gen_puzzles");
    }
    #[test]
    fn serg_benchmark_list() {
        test_file("bench_sudokus/serg_benchmark");
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
        let solutions_str =
            b"812753649943682175675491283154237896369845721287169534521974368438526917796318452";
        assert!(msolve::Sudoku::from(&sudoku).has_single_solution());
        assert_eq!(
            &solution[..],
            &msolve::Sudoku::from(&sudoku)
                .solve_unique()
                .unwrap()
                .to_array()[..]
        );
        assert_eq!(
            &solutions_str[..],
            &msolve::Sudoku::from(&sudoku)
                .solve_unique()
                .unwrap()
                .to_bytes()[..]
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
        assert!(msolve::Sudoku::from(&sudoku).has_single_solution());
        assert_eq!(
            &solution[..],
            &msolve::Sudoku::from(&sudoku)
                .solve_unique()
                .unwrap()
                .to_array()[..]
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
        assert!(msolve::Sudoku::from(&sudoku).has_single_solution());
        assert_eq!(
            &solution[..],
            &msolve::Sudoku::from(&sudoku)
                .solve_unique()
                .unwrap()
                .to_array()[..]
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
        assert!(msolve::Sudoku::from(&sudoku).has_single_solution());
        assert_eq!(
            &solution[..],
            &msolve::Sudoku::from(&sudoku)
                .solve_unique()
                .unwrap()
                .to_array()[..]
        );
    }
    #[test]
    fn empty_has_multiple_solutions() {
        assert_eq!(false, msolve::Sudoku::empty().has_single_solution());
    }
    #[test]
    fn can_find_first_1000_solutions_to_empty() {
        assert_eq!(msolve::Sudoku::empty().count_solutions(1000), 1000);
    }

    #[quickcheck]
    fn random_array_solve(input: Vec<u32>) -> bool {
        let sudoku = msolve::Sudoku::from(input);
        sudoku.solve_one();
        sudoku.to_array();
        sudoku.to_bytes();

        true
    }
    #[quickcheck]
    fn random_string_solve(input: String) -> bool {
        if let Ok(sudoku) = input.parse::<msolve::Sudoku>() {
            sudoku.solve_one();
            sudoku.to_array();
            sudoku.to_bytes();
        }
        true
    }
    #[derive(Clone, Debug)]
    struct Sudoku {
        data: Vec<u8>,
    }
    impl quickcheck::Arbitrary for Sudoku {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let mut data = Vec::<u8>::with_capacity(81);
            for _ in 0..81 {
                data.push(u8::arbitrary(g) % 10);
            }
            Self { data }
        }
    }
    #[quickcheck]
    fn random_sudoku_solve(input: Sudoku) -> bool {
        let sudoku = msolve::Sudoku::from(input.data);
        sudoku.solve_one();
        sudoku.to_array();
        sudoku.to_bytes();

        true
    }
    #[quickcheck]
    fn to_array_returns_inputs_below_10(input: Sudoku) -> bool {
        input.data[..] == msolve::Sudoku::from(&input.data).to_array()[..]
    }

    #[cfg(feature = "generate")]
    #[quickcheck]
    fn generate_from_seed_has_single_solution(input: Sudoku, n: u8, count_steps: bool) -> bool {
        let sudoku = msolve::Sudoku::from(input.data);
        sudoku
            .generate_from_seed(&mut rand::thread_rng(), n as usize, count_steps)
            .0
            .has_single_solution()
    }

    #[cfg(feature = "generate")]
    #[quickcheck]
    fn generated_has_single_solution(count: u8, count_steps: bool) -> bool {
        msolve::Sudoku::generate(rand::thread_rng(), count_steps)
            .take(count as usize)
            .all(|(sudoku, _)| sudoku.has_single_solution())
    }
}
