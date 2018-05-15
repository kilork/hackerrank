const COUNT_ABC: usize = b'z' as usize - b'a' as usize + 1;

fn main() {
    let line: Vec<u8> = read_line()
        .trim_right()
        .bytes()
        .map(|x| x - b'a' as u8)
        .collect();
    let result = is_valid_string(&line);
    println!("{}", result);
}

fn is_valid_string(s: &[u8]) -> &'static str {
    let mut count = [0 as usize; COUNT_ABC];
    for &c in s {
        let c = c as usize;
        count[c] += 1;
    }
    let mut min = 10_000;
    let mut min_count = 0;
    let mut max = 0;
    let mut max_count = 0;
    let mut total_count = 0;
    for &c in count.iter().filter(|&&x| x > 0) {
        total_count += 1;
        if c == min {
            min_count += 1;
        } else if c < min {
            min = c;
            min_count = 1;
        }
        if c == max {
            max_count += 1;
        } else if c > max {
            max = c;
            max_count = 1;
        }
    }
    if max == min
        || (total_count == max_count + min_count
            && ((max == min + 1 && max_count == 1) || (min == 1 && min_count == 1)))
    {
        "YES"
    } else {
        "NO"
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf
}
