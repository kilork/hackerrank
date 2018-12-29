fn main() {
    let count: usize = read_variable();
    for _ in 0..count {
        read_line();
        let data: Vec<usize> = read_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let result = nim_game_1(&data);
        println!("{}", if result { "First" } else { "Second" });
    }
}

fn nim_game_1(data: &[usize]) -> bool {
    let count_nim_sum = data.iter().fold(0, |acc, x| acc ^ x);
    count_nim_sum != 0
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

fn read_variable<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    read_line().parse().unwrap()
}
