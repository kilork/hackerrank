fn main() {
    let first_line = read_line();
    let mut first_line_iter = first_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (n, _m) = (
        first_line_iter.next().unwrap(),
        first_line_iter.next().unwrap(),
    );
    let c: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let variants_count = coin_change_count(n, c);
    println!("{}", variants_count);
}

fn coin_change_count(n: usize, coins: Vec<usize>) -> usize {
    let mut variants_count = vec![0usize; n + 1];
    variants_count[0] = 1;
    for c in coins {
        for i in c..=n {
            variants_count[i] += variants_count[i - c];
        }
    }
    variants_count[n]
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().into()
}
