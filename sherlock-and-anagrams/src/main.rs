fn main() {
    let count: usize = read_line().parse().unwrap();
    for _ in 0..count {
        let line = read_line();
        println!("{}", count_anagrams_substrings(&line));
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn count_anagrams_substrings(s: &str) -> usize {
    let mut count = 0;
    for len in 1..s.len() {
        for i in 0..s.len() - len {
            let mut s1 = String::from(&s[i..i + len]).into_bytes();
            s1.sort_unstable();
            for j in i + 1..s.len() - len + 1 {
                let mut s2 = String::from(&s[j..j + len]).into_bytes();
                s2.sort_unstable();
                if s1 == s2 {
                    count += 1;
                }
            }
        }
    }
    count
}
