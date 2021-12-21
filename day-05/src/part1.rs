pub fn find_door_password(input: &str) -> String {
    let mut password = String::from("");
    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        let hexadecimal = format!("{:x}", hash);
        if hexadecimal.starts_with("00000") {
            println!("hex: {}", hexadecimal);
            password.push(hexadecimal.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }
        i += 1;
    }
    password
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        // assert_eq!(find_door_password("abc"), "18f47a30");
    }
}
