use std::collections::HashSet;

use crate::part1::*;

#[allow(dead_code)]
pub fn intersection(input: &str) -> i32 {
    let commands: Vec<_> = input.split(", ").collect();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut dir: usize = 0;

    for command in commands {
        let (turn, amount) = command.split_at(1);
        let amount: i32 = amount.parse().unwrap();
        dir = take_turn(dir, turn);

        match walk(&mut visited, (x, y), dir, amount) {
            WalkResult::Found(n) => return n,
            WalkResult::Walk((nx, ny)) => {
                x = nx;
                y = ny;
            }
        }
    }

    panic!("Woops, not found")
}

enum WalkResult {
    Found(i32),
    Walk((i32, i32)),
}

fn walk(
    visited: &mut HashSet<(i32, i32)>,
    curr: (i32, i32),
    dir: usize,
    amount: i32,
) -> WalkResult {
    let (x, y) = curr;
    let (dx, dy) = match Direction::VALUES[dir] {
        Direction::North => (0, 1),
        Direction::South => (0, -1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    };

    let (mut cx, mut cy) = (x, y);
    for _ in 0..amount {
        cx += dx;
        cy += dy;
        if visited.contains(&(cx, cy)) {
            return WalkResult::Found(cx.abs() + cy.abs());
        }
        visited.insert((cx, cy));
    }

    WalkResult::Walk((cx, cy))
}

#[cfg(test)]
mod tests {
    use crate::part2::intersection;

    #[test]
    fn examples() {
        assert_eq!(intersection("R8, R4, R4, R8"), 4);
    }
}
