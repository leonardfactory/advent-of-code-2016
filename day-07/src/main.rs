use day_07::{part1::count_tls_ips, part2::count_ssl_ips};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Valid IPs: {}", count_tls_ips(input));
    println!("Valid SSL IPs: {}", count_ssl_ips(input));
    runner.end();
}
