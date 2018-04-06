fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut count = 1;
    for c in buf.chars() {
        if c >= 'A' && c <= 'Z' {
            count += 1;
        }
    }
    println!("{}", count);
}

