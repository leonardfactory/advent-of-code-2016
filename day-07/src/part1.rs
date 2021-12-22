pub fn count_tls_ips(input: &str) -> usize {
    input.split('\n').filter(|&ip| has_tls(ip)).count()
}

pub fn has_tls(ip: &str) -> bool {
    let chars: Vec<_> = ip.chars().enumerate().collect();
    let mut should_take = true;
    let mut abba_in_hypernet = false;
    let mut abba_in_ip = false;
    for (i, c) in chars.iter().take(chars.len() - 3) {
        let slice: String = chars[*i..*i + 4].iter().map(|&(_, sc)| sc).collect();
        match c {
            '[' => should_take = false,
            ']' => should_take = true,
            _ => {
                let mut slice_chars = slice.chars();
                if slice_chars.next() != slice_chars.next()
                    && slice[0..2] == slice[2..4].chars().rev().collect::<String>()
                {
                    if !should_take {
                        abba_in_hypernet = true
                    } else {
                        abba_in_ip = true
                    }
                }
            }
        }
    }
    !abba_in_hypernet && abba_in_ip
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        assert!(has_tls("abba[mnop]qrst"));
        assert!(!has_tls("abcd[bddb]xyyx"));
        assert!(!has_tls("aaaa[qwer]tyui"));
        assert!(has_tls("ioxxoj[asdfgh]zxcvbn"));
    }
}
