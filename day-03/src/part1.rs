use itertools::Itertools;

pub fn count_valid(input: &str) -> usize {
    let triangles = parse_triangles(input);
    triangles
        .iter()
        .filter(|&triangle| is_valid(triangle))
        .count()
}

pub type Triangle = (usize, usize, usize);

pub fn parse_triangles(input: &str) -> Vec<Triangle> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .chunks(5)
                .into_iter()
                .map(|chunk| chunk.collect::<String>().trim().parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn is_valid(triangle: &Triangle) -> bool {
    [triangle.0, triangle.1, triangle.2]
        .into_iter()
        .permutations(3)
        .all(|sides| sides[0] + sides[1] > sides[2])
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_triangles("    4   21  894")[0], (4, 21, 894));
    }

    #[test]
    fn test_example() {
        assert!(!is_valid(&(5, 10, 25)));
        assert!(is_valid(&(5, 10, 8)));
    }
}
