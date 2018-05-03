fn main() {
    let mut s1 = read_chars();
    s1.sort_unstable();
    let mut s2 = read_chars();
    s2.sort_unstable();
    let mut count = 0;
    let mut index_s1 = 0;
    let mut index_s2 = 0;
    
    while index_s1 < s1.len() && index_s2 < s2.len() {
        let a = s1[index_s1];
        let b = s2[index_s2];
        if a < b {
            index_s1 += 1;
            count += 1;
        } else if a > b {
            index_s2 += 1;
            count += 1;
        } else {
            index_s1 += 1;
            index_s2 += 1;
        }
    }
    if index_s1 < s1.len() {
        count += s1.len() - index_s1;
    }
    if index_s2 < s2.len() {
        count += s2.len() - index_s2;
    }
    println!("{}", count);
}

fn read_chars() -> Vec<u8> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().chars().map(|x| x as u8).collect()
}
