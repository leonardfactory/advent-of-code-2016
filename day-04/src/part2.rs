use crate::part1::{valid_rooms, Room};
use std::char;

pub fn find_north_pole_room(input: &str) -> usize {
    let rooms = decode_rooms(input);
    let room = rooms
        .iter()
        .find(|&(name, _sid)| name.contains("pole"))
        .unwrap();

    println!("Room: {}, sid={}", room.0, room.1);
    room.1
}

pub fn decode_rooms(input: &str) -> Vec<(String, usize)> {
    valid_rooms(input).iter().map(decode_room).collect()
}

const RANGE: u32 = 'z' as u32 - 'a' as u32 + 1;
const ZERO: u32 = 'a' as u32;

pub fn decode_room(room: &Room) -> (String, usize) {
    let name = room
        .code
        .chars()
        .map(|c| match c {
            '-' => ' ',
            c => shift_char(c, room.sector_id),
        })
        .collect();

    (name, room.sector_id)
}

fn shift_char(c: char, sector_id: usize) -> char {
    char::from_u32(((c as u32) - ZERO + sector_id as u32) % RANGE + ZERO).unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let room = Room::parse("qzmt-zixmtkozy-ivhz-343[xxxxx]");
        assert_eq!(decode_room(&room).0, "very encrypted name");
    }

    #[test]
    fn test_shift() {
        assert_eq!(shift_char('a', 1), 'b');
        assert_eq!(shift_char('a', 25), 'z');
        assert_eq!(shift_char('a', 27), 'c');
        assert_eq!(shift_char('z', 2), 'b');
    }
}
