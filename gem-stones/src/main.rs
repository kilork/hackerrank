const ELEMENT_COUNT: usize = (b'z' - b'a' + 1) as usize;

fn main() {
    let count: usize = read_line().parse().unwrap();

    let mut counter: [usize; ELEMENT_COUNT] = [0; ELEMENT_COUNT];

    let mut gemstones_count = 0;

    for line_number in 0..count {
        let line = read_line();
        gemstones_count = 0;
        for c in line.chars() {
            let index = c as usize - b'a' as usize;
            if counter[index] == line_number {
                counter[index] = line_number + 1;
                gemstones_count += 1;
            }
        }
    }

    println!("{}", gemstones_count);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
