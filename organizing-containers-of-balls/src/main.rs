fn main() {
    let q = read_usize();
    for _ in 0..q {
        let n = read_usize();
        let possible = check_possibility(StdinRowIterator::new(n));
        println!("{}", if possible { "Possible" } else { "Impossible" });
    }
}

struct StdinRowIterator {
    index: usize,
    count: usize,
}

impl StdinRowIterator {
    fn new(count: usize) -> StdinRowIterator {
        StdinRowIterator { count, index: 0 }
    }
}

impl Iterator for StdinRowIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            self.index += 1;
            Some(read_vec())
        } else {
            None
        }
    }
}

fn check_possibility<I>(rows: I) -> bool
where
    I: IntoIterator<Item = Vec<usize>>,
{
    let containers: Vec<_> = rows.into_iter().collect();
    let mut each_ball_count: Vec<usize> = containers.iter().fold(vec![0; containers.len()], |acc, x| {
        acc.iter().zip(x.iter()).map(|(&a, &b)| a + b).collect()
    });
    each_ball_count.sort();
    let mut total_balls_in_container: Vec<usize> = containers.iter().map(|x| x.iter().sum()).collect();
    total_balls_in_container.sort();

    each_ball_count == total_balls_in_container
}

fn read_vec() -> Vec<usize> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_usize() -> usize {
    read_line().parse().unwrap()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        assert_eq!(true, ::check_possibility(vec![vec![1, 1], vec![1, 1]]));
    }
    #[test]
    fn test_case_2() {
        assert_eq!(false, ::check_possibility(vec![vec![0, 2], vec![1, 1]]));
    }
}
