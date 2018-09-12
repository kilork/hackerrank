fn main() {
    let data: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", beautiful_days_count(data[0], data[1], data[2]));
}

fn beautiful_days_count(from: usize, to: usize, divisor: usize) -> usize {
    (from..to + 1)
        .map(|x| diff(x))
        .filter(|&x| x % divisor == 0)
        .count()
}

fn diff(i: usize) -> usize {
    let rev = reverse_number(i);
    if i > rev {
        i - rev
    } else {
        rev - i
    }
}

fn reverse_number(i: usize) -> usize {
    let mut res = 0;
    let mut rem = i;
    while rem > 0 {
        res *= 10;
        res += rem % 10;
        rem /= 10;
    }
    res
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse_number() {
        assert_eq!(::reverse_number(120), 21);
        assert_eq!(::reverse_number(20), 2);
        assert_eq!(::reverse_number(21), 12);
    }
}
