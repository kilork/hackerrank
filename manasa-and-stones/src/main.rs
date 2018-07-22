fn main() {
    let t = read_usize();
    for _ in 0..t {
        let n = read_usize();
        let a = read_usize();
        let b = read_usize();
        let result = numbers_on_last_stone(n, a.min(b), b.max(a));
        println!(
            "{}",
            result
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

fn numbers_on_last_stone(n: usize, a: usize, b: usize) -> Vec<usize> {
    if a == b {
        return vec![a * (n - 1)];
    }
    (0..n).map(|x| (n - x - 1) * a + x * b).collect()
}

fn read_usize() -> usize {
    read_line().parse().unwrap()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn case_0() {}
}
