fn main() {
    let t: usize = read_line().parse().unwrap();
    for _ in 0..t {
        let _: usize = read_line().parse().unwrap();
        let mut a: Vec<usize> = read_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        println!("{}", if larrys_array(&mut a) { "YES" } else { "NO" });
    }
}

fn larrys_array(a: &mut [usize]) -> bool {
    for i in 0..a.len() {
        let k = i + 1;
        let mut pos = a.iter().position(|&x| x == k).unwrap();
        if pos < i {
            return false;
        }
        while pos > i {
            let diff = pos - i;
            match diff {
                1 => {
                    // 213 -> 132
                    if pos + 1 == a.len() {
                        return false;
                    }
                    let swap = a[pos];
                    a[pos] = a[pos + 1];
                    a[pos + 1] = a[pos - 1];
                    a[pos - 1] = swap;
                    pos -= 1;
                }
                _ => {
                    // 231 -> 123
                    let swap = a[pos];
                    a[pos] = a[pos - 1];
                    a[pos - 1] = a[pos - 2];
                    a[pos - 2] = swap;
                    pos -= 2;
                }
            }
        }
    }
    true
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
