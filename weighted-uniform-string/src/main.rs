use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let original: Vec<u64> = read_line()
        .chars()
        .map(|x| x as u64 - 'a' as u64 + 1)
        .collect();
    let count: usize = read_and_parse();
    for _ in 0..count {
        let weight: u64 = read_and_parse();
        println!("{}", check_weight(&original, weight));
    }
}

fn check_weight(original: &[u64], target_weight: u64) -> &'static str {
    let mut last_char = 0;
    let mut chars = original.iter();
    let mut weight = 0;
    while let Some(&c) = chars.next() {
        if c != last_char {
            weight = 0;
            last_char = c;
        }
        weight += c;
        if weight == target_weight {
            return "Yes";
        }
    }
    "No"
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn read_and_parse<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    read_line().parse().unwrap()
}
