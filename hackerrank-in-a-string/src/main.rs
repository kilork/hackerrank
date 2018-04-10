use std::fmt::Debug;
use std::str::FromStr;

const HACKERRANK: &'static str = "hackerrank";

fn main() {
    let count: usize = read_and_parse();
    for _ in 0..count {
        println!("{}", check_hackerrank());
    }
}

fn check_hackerrank() -> &'static str {
    let line = read_line();
    let mut line_chars = line.chars();
    for c in HACKERRANK.chars() {
        loop {
            match line_chars.next() {
                Some(lc) => {
                    if lc == c {
                        break;
                    }
                }
                None => return "NO",
            }
        }
    }
    "YES"
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
