fn main() {
    let n: usize = read_line().parse().unwrap();
    let b = read_vec_usize();

    assert_eq!(n, b.len());

    if let Some(loaves) = distribute(&b) {
        println!("{}", loaves);
    } else {
        println!("NO");
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

fn read_vec_usize() -> Vec<usize> {
    let line = read_line();
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn distribute(people: &[usize]) -> Option<usize> {
    let mut even_count = 0;
    let mut extra_loaves = 0;

    for loaves in people {
        let inside_odd_sequence = even_count % 2 == 1;
        let is_loaves_odd = loaves % 2;
        even_count += is_loaves_odd;
        if inside_odd_sequence {
            extra_loaves += 2;
        }
    }

    if even_count % 2 == 1 {
        return None;
    }

    Some(extra_loaves)
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_1() {
        assert_eq!(Some(4), ::distribute(&[2, 3, 4, 5, 6]));
    }

    #[test]
    fn case_2() {
        assert_eq!(None, ::distribute(&[1, 2]));
    }
}
