use core::panic;

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const VALUES: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];
}

#[allow(dead_code)]
pub fn distance(input: &str) -> i32 {
    let commands: Vec<_> = input.split(", ").collect();
    let mut x = 0;
    let mut y = 0;
    let mut dir: usize = 0;

    for command in commands {
        let (turn, amount) = command.split_at(1);
        let amount: i32 = amount.parse().unwrap();
        dir = take_turn(dir, turn);

        match Direction::VALUES[dir] {
            Direction::North => y += amount,
            Direction::South => y -= amount,
            Direction::East => x += amount,
            Direction::West => x -= amount,
        }
    }

    x.abs() + y.abs()
}

pub fn take_turn(dir: usize, turn: &str) -> usize {
    match turn {
        "R" => (dir as i32 + 1).rem_euclid(4).try_into().unwrap(),
        "L" => (dir as i32 - 1).rem_euclid(4).try_into().unwrap(),
        _ => panic!("woops"),
    }
}

#[cfg(test)]
mod tests {
    use crate::part1::distance;

    #[test]
    fn examples() {
        assert_eq!(distance("R2, L3"), 5);
        assert_eq!(distance("R5, L5, R5, R3"), 12);
    }
}
