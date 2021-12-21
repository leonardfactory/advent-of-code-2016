use itertools::Itertools;

pub fn valid_rooms_sum_sectors(input: &str) -> usize {
    valid_rooms(input).iter().map(|r| r.sector_id).sum()
}

pub fn valid_rooms(input: &str) -> Vec<Room> {
    input
        .split('\n')
        .map(Room::parse)
        .filter(is_valid_room)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub code: String,
    pub sector_id: usize,
    pub checksum: String,
}

impl Room {
    pub fn parse(input: &str) -> Self {
        let (code, checksum) = input.split('[').collect_tuple().unwrap();
        Self {
            code: code[..code.len() - 4].to_string(),
            sector_id: code[code.len() - 3..].parse().unwrap(),
            checksum: checksum[..checksum.len() - 1].to_string(),
        }
    }
}

fn is_valid_room(room: &Room) -> bool {
    let checksum: String = room
        .code
        .replace("-", "")
        .chars()
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| (c, group.count()))
        .sorted_by(|&(a, a_len), &(b, b_len)| {
            if a_len != b_len {
                b_len.cmp(&a_len)
            } else {
                a.cmp(&b)
            }
        })
        .map(|(char, _)| char)
        .take(5)
        .collect();

    // println!("room={:?}, checksum={}", room, checksum);
    checksum == room.checksum
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            Room::parse("aaaaa-bbb-z-y-x-123[abxyz]"),
            Room {
                code: "aaaaa-bbb-z-y-x".to_string(),
                sector_id: 123,
                checksum: "abxyz".to_string()
            }
        );
    }

    #[test]
    fn test_valid() {
        assert_eq!(valid_rooms_sum_sectors(include_str!("../test.txt")), 1514);
    }
}
