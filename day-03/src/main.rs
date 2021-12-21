use day_03::{part1::count_valid, part2::count_valid_in_columns};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Valid: {}", count_valid(input));
    println!("Valid in columns: {}", count_valid_in_columns(input));
    runner.end();
}
