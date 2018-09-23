fn main() {
    let count: usize = read_line().parse().unwrap();
    for _ in 0..count {
        let data: Vec<usize> = read_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        println!("{}", save_the_prisoner(data[0], data[1], data[2]));
    }
}

fn save_the_prisoner(n: usize, m: usize, s: usize) -> usize {
    let result = (s + m - 1) % n;
    if result == 0 {
        n
    } else {
        result
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
    fn test_01() {
        assert_eq!(3, ::save_the_prisoner(4, 6, 2));
        assert_eq!(2, ::save_the_prisoner(5, 2, 1));
        assert_eq!(3, ::save_the_prisoner(5, 2, 2));
        assert_eq!(6, ::save_the_prisoner(7, 19, 2));
    }
}