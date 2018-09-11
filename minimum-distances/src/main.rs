fn main() {
    let _len: usize = read_line().parse().unwrap();
    let numbers: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    if let Some(distance) = minimum_distance(&numbers) {
        println!("{}", distance);
    } else {
        println!("-1");
    }
}

fn minimum_distance(numbers: &[usize]) -> Option<usize> {
    let mut min = numbers.len();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] == numbers[j] && j - i < min {
                min = j - i;
                if min == 1 {
                    return Some(1);
                }
            }
        }
    }
    if min != numbers.len() {
        Some(min)
    } else {
        None
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
