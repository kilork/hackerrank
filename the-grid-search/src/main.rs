#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<String>,
}

impl Matrix {
    fn from_stdin() -> Matrix {
        let (rows, cols) = read_usize_pair();
        let data = (0..rows).map(|_| read_line()).collect();
        Matrix { rows, cols, data }
    }
}

fn main() {
    let t: usize = read_line().parse().unwrap();
    for _ in 0..t {
        let matrix_a = Matrix::from_stdin();
        let matrix_b = Matrix::from_stdin();

        let matrix_pos: Option<(usize, usize)> = find_matrix(&matrix_a, &matrix_b);

        println!("{}", if matrix_pos.is_some() { "YES" } else { "NO" });
    }
}

fn find_matrix(source: &Matrix, pattern: &Matrix) -> Option<(usize, usize)> {
    if pattern.rows > source.rows || pattern.cols > source.cols {
        return None;
    }

    if let Some(first_line) = pattern.data.first() {
        for i in 0..source.rows - pattern.rows + 1 {
            let source_line: &str = &source.data[i];

            'm: for m in 0..source_line.len() - first_line.len() + 1 {
                for row in 0..pattern.rows {
                    let row_substr = &source.data[i + row][m..m + first_line.len()];
                    let pattern = &pattern.data[row];

                    if row_substr != pattern {
                        continue 'm;
                    }
                }

                return Some((i, m));
            }
        }
    }

    None
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

fn read_usize_pair() -> (usize, usize) {
    let line = read_line();
    let mut line = line.split_whitespace();
    (
        line.next().unwrap().parse().unwrap(),
        line.next().unwrap().parse().unwrap(),
    )
}
