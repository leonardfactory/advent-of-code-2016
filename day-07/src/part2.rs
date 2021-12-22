use itertools::Itertools;

pub fn count_ssl_ips(input: &str) -> usize {
    input.split('\n').filter(|&ip| has_ssl(ip)).count()
}

pub fn has_ssl(ip: &str) -> bool {
    let subnets: Vec<_> = ip.split(['[', ']'].as_ref()).enumerate().collect();

    let supernets: Vec<_> = subnets
        .iter()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, s)| s)
        .collect();
    let hypernets: Vec<_> = subnets
        .iter()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, s)| s)
        .collect();

    let abas: Vec<_> = supernets
        .iter()
        .flat_map(|supernet| find_abas(supernet))
        .collect();

    abas.iter().any(|code| {
        hypernets
            .iter()
            .any(|hypernet| hypernet.contains(&get_bab_code(code)))
    })
}

fn find_abas(supernet: &str) -> Vec<String> {
    supernet
        .chars()
        .take(supernet.len() - 2)
        .enumerate()
        .map(|(i, _c)| supernet[i..i + 3].to_owned())
        .filter(|aba| {
            let (a1, b, a2) = aba.chars().collect_tuple().unwrap();
            a1 == a2 && a1 != b
        })
        .collect()
}

fn get_bab_code(aba: &str) -> String {
    let (a, b, _) = aba.chars().collect_tuple().unwrap();
    format!("{}{}{}", b, a, b)
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        assert!(has_ssl("aba[bab]xyz"));
        assert!(!has_ssl("xyx[xyx]xyx"));
        assert!(has_ssl("aaa[kek]eke"));
        assert!(has_ssl("zazbz[bzb]cdb"));
    }
}
