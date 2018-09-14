use std::fmt;
use std::ops::MulAssign;

fn main() {
    let n: u32 = read_line().parse().unwrap();
    println!("{}", factorial(n));
}

fn factorial(n: u32) -> BigInteger {
    let mut f = BigInteger::from(1);
    for i in 2..n + 1 {
        f *= i;
    }
    f
}

const MAX_BIG_INTEGER_CELL: u32 = 1_000_000;

struct BigInteger {
    a: Vec<u32>,
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = String::new();
        let mut i = self.a.len() - 1;
        b.push_str(&self.a[i].to_string());
        while i > 0 {
            i -= 1;
            b.push_str(&format!("{:06}", self.a[i]));
        }
        write!(f, "{}", b)
    }
}

impl MulAssign<u32> for BigInteger {
    fn mul_assign(&mut self, rhs: u32) {
        let mut i = 0;
        let mut acc = 0;
        while i < self.a.len() {
            let mut b = self.a[i] * rhs + acc;
            acc = 0;
            while b >= MAX_BIG_INTEGER_CELL {
                acc += 1;
                b -= MAX_BIG_INTEGER_CELL;
            }
            self.a[i] = b;
            i += 1;
        }
        if acc != 0 {
            self.a.push(acc);
        }
    }
}

impl From<u32> for BigInteger {
    fn from(value: u32) -> BigInteger {
        BigInteger {
            a: vec![value as u32],
        }
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_01() {}
}
