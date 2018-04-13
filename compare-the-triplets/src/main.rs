fn main() {
    let alice = read_numbers();
    let bob = read_numbers();
    let mut alice_score = 0;
    let mut bob_score = 0;
    alice.iter().zip(bob.iter()).for_each(|(&a, &b)| {
        if a > b {
            alice_score += 1;
        } else if b > a {
            bob_score += 1;
        }
    });
    println!("{} {}", alice_score, bob_score);
}

fn read_numbers() -> Vec<u64> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
