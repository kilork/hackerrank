fn main() {
    let a = read_line();
    let b = read_line();
    let k = read_line().parse().unwrap();

    println!("{}", if check_turns(&a, &b, k) { "Yes" } else { "No" });
}

fn check_turns(a: &str, b: &str, k: usize) -> bool {
    if a.len() + b.len() <= k {
        return true;
    }
    let mut common_len = 0;
    let mut ai = a.as_bytes().iter();
    let mut bi = b.as_bytes().iter();
    while let (Some(a), Some(b)) = (ai.next(), bi.next()) {
        if a == b {
            common_len += 1;
        } else {
            break;
        }
    }
    if k < a.len() + b.len() - 2 * common_len {
        return false;
    }
    let remaining = k + 2 * common_len - a.len() - b.len();
    remaining % 2 == 0
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        assert!(::check_turns("hackerhappy", "hackerrank", 9));
        assert!(!::check_turns("hackerhappy", "hackerrank", 10));
        assert!(::check_turns("hackerhappy", "hackerrank", 11));
    }

    #[test]
    fn test_case_2() {
        assert!(::check_turns("aba", "aba", 7));
    }

    #[test]
    fn test_case_3() {
        assert!(!::check_turns("ashley", "ash", 2));
    }
}
