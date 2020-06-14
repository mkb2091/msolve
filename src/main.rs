mod lib;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::with_capacity(82);
    let solver = lib::Solver::new();
    let stdout = std::io::stdout();
    let mut output_handle = stdout.lock();
    while let Ok(result) = input.read_line(&mut buffer) {
        if result == 0 {
            break;
        }
        if let Some(solution) = solver.solve_string(&buffer) {
            let _ = output_handle.write_all(
                &solution
                    .iter()
                    .map(|x| x.to_string().as_bytes()[0])
                    .collect::<Vec<u8>>(),
            );
            let _ = output_handle.write_all(b"\n");
        }
        buffer.clear();
    }
}
