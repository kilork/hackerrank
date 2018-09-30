fn main() {
    let line = read_vec();
    let (_n, k) = (line[0], line[1]);
    let ar = read_vec();
    println!("{}", count_divisible_sum_pairs(k, &ar));
}

fn count_divisible_sum_pairs(k: usize, ar: &[usize]) -> usize {
    let mut count = 0;
    for (i, ar_i) in ar.iter().enumerate() {
        for ar_j in &ar[i + 1..] {
            if (ar_i + ar_j) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn read_vec() -> Vec<usize> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
