const ABC_COUNT: usize = 'z' as usize - 'a' as usize + 1;

fn main() {
    let result = analyze_anagram();
    println!("{}", result);
}

fn analyze_anagram() -> &'static str {
    let mut counter = [0; ABC_COUNT];
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().chars()
        .for_each(|c| counter[c as usize - 'a' as usize] += 1);
    let mut mod_1_count = 0;
    for count in counter.iter() {
        if count % 2 == 1 {
            mod_1_count += 1;
        }
        if mod_1_count == 2 {
            return "NO";
        }
    }
    "YES"
}
