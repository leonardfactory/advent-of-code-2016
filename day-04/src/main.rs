use day_04::{part1::valid_rooms_sum_sectors, part2::find_north_pole_room};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Valids sector IDS: {}", valid_rooms_sum_sectors(input));
    println!("Room: {}", find_north_pole_room(input));
    runner.end();
}
