use day_01::{part1::distance, part2::intersection};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    println!("Distance: {}", distance(include_str!("../input.txt")));
    println!("Location: {}", intersection(include_str!("../input.txt")));
    runner.end();
}
