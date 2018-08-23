fn main() {
    let n = read_usize();
    let p = read_usize();

    println!("{}", min_pages_to_turn(n, p));
}

fn min_pages_to_turn(n: usize, p: usize) -> usize {
    p.min(n - p) / 2
}

fn read_usize() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_01() {
        assert_eq!(1, ::min_pages_to_turn(6, 2));
    }

    #[test]
    fn case_02() {
        assert_eq!(0, ::min_pages_to_turn(5, 4));
    }
}
