use std::cmp::{max, min};

pub fn find_code(input: &str) -> String {
    parse(input)
        .iter()
        .fold(("".to_owned(), (1, 1)), |(str, pos), seq| {
            let next_pos = seq.iter().fold(pos, |pos, &delta| move_by(pos, delta));
            println!("next_pos={:?}, str={}, pos={:?}", next_pos, str, pos);
            (str + &button_value(next_pos).to_string(), next_pos)
        })
        .0
}

fn move_by(pos: (i32, i32), delta: (i32, i32)) -> (i32, i32) {
    (
        max(min(pos.0 + delta.0, 2), 0),
        max(min(pos.1 + delta.1, 2), 0),
    )
}

fn button_value(pos: (i32, i32)) -> i32 {
    pos.0 + 1 + pos.1 * 3
}

pub fn parse(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => (0, -1),
                    'L' => (-1, 0),
                    'R' => (1, 0),
                    'D' => (0, 1),
                    _ => panic!("?"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_number_code() {
        assert_eq!(button_value((2, 1)), 6);
    }

    #[test]
    fn example_input() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(find_code(input), "1985");
    }
}
