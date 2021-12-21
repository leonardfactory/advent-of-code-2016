use std::cmp::{max, min};

use crate::part1::parse;

pub fn find_hexadecimal_code(input: &str) -> String {
    parse(input)
        .iter()
        .fold(("".to_owned(), (0, 2)), |(str, pos), seq| {
            let next_pos = seq.iter().fold(pos, |pos, &delta| move_by(pos, delta));
            println!("next_pos={:?}, str={}, pos={:?}", next_pos, str, pos);
            (str + &button_value(next_pos), next_pos)
        })
        .0
}

fn move_by(pos: (i32, i32), delta: (i32, i32)) -> (i32, i32) {
    let x_min = (5 - STEPS[pos.1 as usize]) / 2;
    let x_max = STEPS[pos.1 as usize] + x_min - 1;
    let y_min = (5 - STEPS[pos.0 as usize]) / 2;
    let y_max = STEPS[pos.0 as usize] + y_min - 1;
    (
        max(min(pos.0 + delta.0, x_max), x_min),
        max(min(pos.1 + delta.1, y_max), y_min),
    )
}

const STEPS: [i32; 5] = [1, 3, 5, 3, 1];

fn button_value(pos: (i32, i32)) -> String {
    format!(
        "{:X}",
        (pos.0 - (5 - STEPS[pos.1 as usize]) / 2)
            + 1
            + STEPS.iter().take(pos.1 as usize).sum::<i32>()
    )
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn example_input() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(find_hexadecimal_code(input), "5DB3");
    }
}
