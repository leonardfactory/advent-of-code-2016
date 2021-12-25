use lazy_static::lazy_static;
use regex::Regex;

pub fn decompressed_size(input: &str) -> usize {
    lazy_static! {
        static ref MARKER_RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    }

    let mut size = 0;
    let mut marker = String::new();
    let mut chars = input.char_indices();
    while let Some((_, c)) = chars.next() {
        marker.push(c);
        size += 1;
        if let Some(capture) = MARKER_RE.captures(&marker) {
            // println!("matched: {:?}, char={}", capture, c);
            let marker_unit = capture.get(0).unwrap().as_str();
            let count = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let factor = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            chars.nth(count - 1);
            size -= marker_unit.len();
            size += factor * count;
            marker.clear();
        }
    }
    size
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        assert_eq!(decompressed_size("ADVENT"), 6);
        assert_eq!(decompressed_size("A(1x5)BC"), 7);
        assert_eq!(decompressed_size("(3x3)XYZ"), 9);
        assert_eq!(decompressed_size("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(decompressed_size("(6x1)(1x3)A"), 6);
        assert_eq!(decompressed_size("X(8x2)(3x3)ABCY"), 18);
    }
}
