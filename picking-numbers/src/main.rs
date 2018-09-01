fn main() {
    let _: usize = read_line().parse().unwrap();
    let data: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", picking_numbers(&data));
}

fn picking_numbers(data: &[usize]) -> usize {
    let mut max_count = 0;
    for &a in data {
        let mut count = 0;
        for &b in data {
            if a <= b && b - a <= 1 {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
        }
    }
    max_count
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn case_0() {
        assert_eq!(5, ::picking_numbers(&[1, 2, 2, 3, 1, 2]));
    }

    #[test]
    fn case_1() {
        assert_eq!(3, ::picking_numbers(&[4, 6, 5, 3, 3, 1]));
    }
}
