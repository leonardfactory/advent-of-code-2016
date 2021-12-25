use day_09::{part1::decompressed_size, part2::decompressed_size_two};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Decompressed: {}", decompressed_size(input));
    println!("Decompressed V2©: {}", decompressed_size_two(input));
    runner.end();
}
