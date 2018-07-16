fn main() {
    let (n, m) = read_pair();
    let mut data = read_array_usize();
    let solution = find_maximum(n, m, &mut data);
    println!("{}", solution);
}

fn find_maximum(n: usize, m: usize, stations: &mut [usize]) -> usize {
    stations.sort();
    let mut max_distance = stations[0];
    for i in 1..m {
        let distance = (stations[i] - stations[i - 1]) / 2;
        if distance > max_distance {
            max_distance = distance;
        }
    }
    let last_station = stations[m - 1];
    let last_distance = n - last_station - 1;
    if last_distance > max_distance {
        max_distance = last_distance;
    }
    max_distance
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

fn read_array_usize() -> Vec<usize> {
    let line = read_line();
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_pair() -> (usize, usize) {
    let data = read_array_usize();
    (data[0], data[1])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_01() {
        assert_eq!(2, ::find_maximum(5, 2, &mut [0, 4]));
    }
    #[test]
    fn test_case_02() {
        assert_eq!(0, ::find_maximum(6, 6, &mut [0, 1, 2, 4, 3, 5]));
    }
}
