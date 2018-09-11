fn main() {
    let weights: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let word: Vec<char> = read_line().chars().collect();
    println!("{}", calculate_selection_area(&weights, &word));
}

fn calculate_selection_area(weights: &Vec<usize>, word: &Vec<char>) -> usize {
    word.iter()
        .map(|&x| weights[x as usize - 'a' as usize])
        .max()
        .map(|x| x * word.len())
        .unwrap()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}
