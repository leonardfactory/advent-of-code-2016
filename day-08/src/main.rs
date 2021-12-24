use day_08::part1::update_screen;
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Lit pixels: {}", update_screen(input));
    runner.end();
}
