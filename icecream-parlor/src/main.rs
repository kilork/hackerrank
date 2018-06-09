type Money = u16;
fn main() {
    let t: usize = read_usize();
    for _ in 0..t {
        let m: Money = read_money();
        let n = read_usize();
        let c: Vec<Money> = read_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (index_a, index_b) = icecream_parlor(&m, n, &c);
        println!("{} {}", index_a, index_b);
    }
}

fn icecream_parlor(m: &Money, n: usize, c: &[Money]) -> (usize, usize) {
    for i in 0..n - 1 {
        let a = c[i];
        for j in i + 1..n {
            if a + c[j] == *m {
                return (i + 1, j + 1);
            }
        }
    }
    panic!();
}

fn read_money() -> Money {
    read_line().parse().unwrap()
}

fn read_usize() -> usize {
    read_line().parse().unwrap()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
