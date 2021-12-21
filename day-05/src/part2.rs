pub fn find_door_secure_password(input: &str) -> String {
    let mut password = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut i = 0;
    let mut filled = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        let hexadecimal = format!("{:x}", hash);
        i += 1;
        if hexadecimal.starts_with("00000") {
            println!("hex: {}", hexadecimal);
            let index = match hexadecimal.chars().nth(5).unwrap().to_digit(10) {
                Some(n) if n <= 7 && password[n as usize] == ' ' => n as usize,
                _ => continue,
            };

            password[index] = hexadecimal.chars().nth(6).unwrap();
            filled += 1;
            if filled == 8 {
                break;
            }
        }
    }
    password.iter().collect()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        assert_eq!(find_door_secure_password("abc"), "05ace8e3");
    }
}
