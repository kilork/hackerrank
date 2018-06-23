fn main() {
    let line = read_line();
    let mut words = line.split_whitespace();
    if let Some(n) = words.next() {
        let n = n.parse().unwrap();
        let data: Vec<String> = (0..n).map(|_| read_line()).collect();
        let (max, count) = find_max(n, &data);
        println!("{}", max);
        println!("{}", count);
    }
}

fn count_ones(a: &str, b: &str) -> usize {
    a.bytes()
        .zip(b.bytes())
        .filter(|&(x, y)| x == b'1' || y == b'1')
        .count()
}

fn find_max(n: usize, data: &[String]) -> (usize, u64) {
    let mut max = 0;
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let pair = count_ones(&data[i], &data[j]);
            if pair > max {
                count = 0;
                max = pair;
            }
            if pair == max {
                count += 1;
            }
        }
    }
    (max, count)
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
