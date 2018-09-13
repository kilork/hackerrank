use std::ops::Add;

fn main() {
    let n: u8 = read_line().parse().unwrap();
    println!("{}", factorial(n));
}

fn factorial(n: u8) -> BigInteger {
    BigInteger {}
}

struct BigInteger {
    a: Vec<u32>,
}

impl Add for BigInteger {
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into();
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_01() {
    }
}