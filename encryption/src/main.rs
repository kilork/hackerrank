fn main() {
    let s = read_line();
    println!("{}", encrypt(&s));
}

fn encrypt(s: &str) -> String {
    let lim = (s.len() as f64).sqrt();
    let rows = lim.floor() as usize;
    let cols = lim.ceil() as usize;
    let (rows, cols) = if rows * rows >= s.len() {
        (rows, rows)
    } else if rows * cols >= s.len() {
        (rows, cols)
    } else {
        (cols, cols)
    };
    let mut result = Vec::new();
    for j in 0..cols {
        let mut word = String::new();
        for i in 0..rows {
            let index = i * cols + j;
            if let Some(c) = s.get(index..index + 1) {
                word += c;
            }
        }
        result.push(word);
    }

    result.join(" ")
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
