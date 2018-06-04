extern crate morgan_and_a_string_bfs;

use morgan_and_a_string_bfs::minimal_sequence_finder;

fn main() {
    let count: usize = read_line().parse().unwrap();

    for _ in 0..count {
        let a = read_line();
        let b = read_line();
        println!("{}", minimal_sequence_finder(&a, &b));
    }
}
fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_string()
}
