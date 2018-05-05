fn main() {
    let count: usize = read_line().parse().unwrap();
    for _ in 0..count {
        let result = process_two_strings();
        println!("{}", result);
    }
}

fn process_two_strings() -> &'static str {
    let mut line1: Vec<u8> = read_line().bytes().collect();
    line1.sort_unstable();
    line1.dedup();
    let mut line2: Vec<u8> = read_line().bytes().collect();
    line2.sort_unstable();
    if let Some(_) = line1.iter().find(|&x| {
        if let Ok(_) = line2.binary_search(x) {
            true
        } else {
            false
        }
    }) {
        "YES"
    } else {
        "NO"
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
