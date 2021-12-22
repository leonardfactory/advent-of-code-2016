use std::collections::HashMap;

pub fn recover_message(input: &str) -> String {
    let mut lines = input.split('\n');
    let mut chars: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| {
            let mut map = HashMap::<char, usize>::new();
            map.insert(x, 1);
            map
        })
        .collect();

    for line in lines {
        line.chars().enumerate().for_each(|(i, k)| {
            let entry = chars[i].entry(k).or_insert(0);
            *entry += 1;
        })
    }

    chars
        .iter()
        .map(|map| {
            map.iter()
                .max_by(|a, b| a.1.cmp(b.1))
                .map(|(&k, _v)| k)
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        assert_eq!(recover_message(include_str!("../test.txt")), "easter");
    }
}
