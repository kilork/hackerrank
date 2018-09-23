fn main() {
    let count: usize = read_line().parse().unwrap();
    let mut data: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", equalize_the_array(&mut data[..count]));
}

fn equalize_the_array(data: &mut [usize]) -> usize {
    data.sort();
    let mut max_count = 1;
    let mut i = 1;
    let mut count = 1;
    let mut d = data[0];
    while i < data.len() {
        if d != data[i] {
            d = data[i];
            if count > max_count {
                max_count = count;
                if max_count >= data.len() - i {
                    break;
                }
            }
            count = 0;
        }
        count += 1;
        i += 1;
    }
    if count > max_count {
        max_count = count;
    }
    data.len() - max_count
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_01() {
        assert_eq!(2, ::equalize_the_array(&mut [3, 3, 1, 2, 3]));
        assert_eq!(2, ::equalize_the_array(&mut [2, 2, 3, 3, 3]));
        assert_eq!(0, ::equalize_the_array(&mut [3, 3, 3, 3, 3]));
        assert_eq!(1, ::equalize_the_array(&mut [1, 2]));
    }
}
