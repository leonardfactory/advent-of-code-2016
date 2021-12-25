use lazy_static::lazy_static;
use regex::Regex;

pub fn decompressed_size_two(input: &str) -> usize {
    lazy_static! {
        static ref MARKER_RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    }

    let mut size = 0;
    let mut marker = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        marker.push(c);
        size += 1;
        if let Some(capture) = MARKER_RE.captures(&marker) {
            // println!("matched: {:?}, char={}", capture, c);
            let marker_unit = capture.get(0).unwrap().as_str();
            let count = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let factor = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let compressed: String = chars.by_ref().take(count).collect();
            let nested_size = decompressed_size_two(&compressed);

            size -= marker_unit.len();
            size += factor * nested_size;
            marker.clear();
        }
    }
    size
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        assert_eq!(
            decompressed_size_two("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
        assert_eq!(decompressed_size_two("(3x3)XYZ"), 9);
        assert_eq!(
            decompressed_size_two("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
    }
}
