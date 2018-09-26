fn main() {
    let n: usize = read_line().parse().unwrap();
    let mut cumulative = 0;
    let mut liked = 2;
    for _ in 0..n {
        cumulative += liked;
        liked = liked * 3 / 2;
    }
    println!("{}", cumulative);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
