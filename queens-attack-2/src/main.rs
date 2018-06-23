fn main() {
    let (n, k) = read_i32_pair();
    let queen = read_position();
    let targets = queens_attack_2(n, &queen, ObstaclesStdin::new(k as usize));
    println!("{}", targets);
}

type Position = (i32, i32);

struct ObstaclesStdin {
    index: usize,
    count: usize,
}

impl ObstaclesStdin {
    fn new(k: usize) -> ObstaclesStdin {
        ObstaclesStdin { index: 0, count: k }
    }
}

impl Iterator for ObstaclesStdin {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.count {
            self.index += 1;
            Some(read_position())
        } else {
            None
        }
    }
}

fn position_indexes(p: &(i32, i32)) -> (i32, i32, usize) {
    let sign_x = p.0.signum();
    let sign_y = p.1.signum();
    let index = (sign_y + 1) * 3 + sign_x + 1;
    (sign_x, sign_y, index as usize)
}

fn queens_attack_2<I>(n: i32, queen: &Position, obstacles: I) -> i32
where
    I: IntoIterator<Item = Position>,
{
    let mut mins = [0i32; 9];
    let q = (queen.0 - 1, queen.1 - 1);
    let n = n - 1;
    mins[0] = q.0.min(q.1);
    mins[1] = q.1;
    mins[2] = (n - q.0).min(q.1);
    mins[3] = q.0;
    mins[5] = n - q.0;
    mins[6] = q.0.min(n - q.1);
    mins[7] = n - q.1;
    mins[8] = (n - q.0).min(n - q.1);
    obstacles
        .into_iter()
        .map(|x| (x.0 - queen.0, x.1 - queen.1))
        .filter(|x| x.0 == 0 || x.1 == 0 || x.0 == x.1 || x.0 == -x.1)
        .for_each(|x| {
            let (sign_x, sign_y, index) = position_indexes(&x);
            let current = mins[index];
            let proposed = if x.0 == 0 { x.1 * sign_y } else { x.0 * sign_x } - 1;
            if proposed < current {
                mins[index] = proposed;
            }
        });
    mins.iter().sum()
}

fn read_position() -> Position {
    read_i32_pair()
}

fn read_i32_pair() -> (i32, i32) {
    let line = read_line();
    let mut words = line.split_whitespace();
    (
        words.next().unwrap().parse().unwrap(),
        words.next().unwrap().parse().unwrap(),
    )
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn case_1() {
        assert_eq!(9, ::queens_attack_2(4, &(4, 4), ::ObstaclesStdin::new(0)));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            10,
            ::queens_attack_2(5, &(4, 3), vec![(5, 5), (4, 2), (2, 3)].into_iter())
        );
    }
}
