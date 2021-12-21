use day_05::{part1::find_door_password, part2::find_door_secure_password};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = "wtnhxymk";
    println!("Password is: {}", find_door_password(input));
    println!("Secure Password is: {}", find_door_secure_password(input));
    runner.end();
}
