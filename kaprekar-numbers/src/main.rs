use std::ops::RangeInclusive;

fn main() {
    let p = read_u64();
    let q = read_u64();
    let kaprikan_numbers = find_kaprikan_numbers(p..=q);
    println!(
        "{}",
        if kaprikan_numbers.is_empty() {
            "INVALID RANGE".into()
        } else {
            kaprikan_numbers
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }
    );
}

fn find_kaprikan_numbers(pq: RangeInclusive<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    for i in pq {
        let i_str = i.to_string();
        let digits_count = i_str.len();
        let ii = i * i;
        let ii_str = ii.to_string();
        let mid = ii_str.len() - digits_count;
        let l: u64 = ii_str[..mid].parse().unwrap_or(0);
        let r: u64 = ii_str[mid..].parse().unwrap();
        if l + r == i {
            result.push(i);
        }
    }
    result
}

fn read_u64() -> u64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        assert_eq!(vec![1, 9, 45, 55, 99], ::find_kaprikan_numbers(1..=100));
    }
}
