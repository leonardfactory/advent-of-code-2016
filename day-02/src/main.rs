use day_02::{part1::find_code, part2::find_hexadecimal_code};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    println!("Code: {}", find_code(include_str!("../input.txt")));
    println!(
        "Code: {}",
        find_hexadecimal_code(include_str!("../input.txt"))
    );
    runner.end();
}
