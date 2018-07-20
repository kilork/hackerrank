fn main() {
    let n: usize = read_line().parse().unwrap();

    let mut map = vec![];

    for _ in 0..n {
        map.push(read_line().bytes().collect());
    }

    let cavities_map = mark_cavity(&map);

    for line in cavities_map.iter()
        .map(|x| x.iter().map(|&x| x as char).collect::<String>())
    {
        println!("{}", line);
    }
}

fn mark_cavity(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut target = map.clone();

    for i in 1..map.len() - 1 {
        let line = &map[i];

        for j in 1..line.len() - 1 {
            let c = line[j];

            if c > map[i][j - 1] && c > map[i][j + 1] && c > map[i - 1][j] && c > map[i + 1][j] {
                target[i][j] = b'X';
            }
        }
    }

    target
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {
    fn from_str(data: &[&str]) -> Vec<Vec<u8>> {
        data.iter().map(|x| x.bytes().collect()).collect()
    }

    #[test]
    fn case_1() {
        let input = from_str(&vec!["1112", "1912", "1892", "1234"]);
        let expected = from_str(&vec!["1112", "1X12", "18X2", "1234"]);
        assert_eq!(expected, ::mark_cavity(&input));
    }

    #[test]
    fn case_2() {
        let input = from_str(&vec!["989", "191", "111"]);
        let expected = from_str(&vec!["989", "1X1", "111"]);
        assert_eq!(expected, ::mark_cavity(&input));
    }
}
