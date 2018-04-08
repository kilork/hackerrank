//! Hackerrank problem: https://www.hackerrank.com/challenges/mars-exploration/problem

/// SOS array const
const SOS: [char; 3] = ['S', 'O', 'S'];

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let chars = buf.trim().chars();
    let mut mutations_count = 0;
    for (index, c) in chars.enumerate() {
        if c != SOS[index % 3] {
            mutations_count += 1;
        }
    }
    println!("{}", mutations_count);
}
