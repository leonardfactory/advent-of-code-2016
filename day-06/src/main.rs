use day_06::{part1::recover_message, part2::recover_message_least};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Message: {}", recover_message(input));
    println!("Message least: {}", recover_message_least(input));
    runner.end();
}
