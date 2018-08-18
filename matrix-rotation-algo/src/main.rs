fn main() {
    let (rows, cols, moves) = read_rows_cols_moves();
    let matrix: Vec<Vec<usize>> = (0..rows).map(|_| read_usize_array()).collect();
    let result_matrix = matrix_rotate(&matrix, rows, cols, moves);
    println!(
        "{}",
        result_matrix
            .iter()
            .map(|x| x
                .iter()
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

type Matrix = Vec<Vec<usize>>;

fn matrix_rotate(matrix: &Matrix, rows: usize, cols: usize, moves: usize) -> Matrix {
    let unassembled = unassemble(&matrix);
    let rotated: Matrix = unassembled.iter().map(|x| rotate(&x, moves)).collect();
    assemble(&rotated, rows, cols)
}

fn rotate(line: &[usize], moves: usize) -> Vec<usize> {
    let mut result = vec![];
    (0..line.len()).for_each(|x| result.push(line[(x + moves) % line.len()]));
    result
}

fn unassemble(matrix: &Matrix) -> Matrix {
    let mut result = vec![];
    let (mut top, mut left, mut right, mut bottom) = (0, 0, matrix[0].len() - 1, matrix.len() - 1);
    while top <= bottom && left <= right {
        let mut line: Vec<usize> = vec![];
        line.extend_from_slice(&matrix[top][left..=right]);
        for i in (top + 1)..bottom {
            line.push(matrix[i][right]);
        }
        if top != bottom {
            line.extend(matrix[bottom][left..=right].iter().rev());
        }
        if left != right {
            for i in ((top + 1)..bottom).rev() {
                line.push(matrix[i][left]);
            }
        }
        result.push(line);

        top += 1;
        bottom -= 1;
        left += 1;
        right -= 1;
    }
    result
}

fn assemble(parts: &Matrix, rows: usize, cols: usize) -> Matrix {
    let mut result = vec![vec![0; cols]; rows];
    let (mut top, mut left, mut right, mut bottom) = (0, 0, cols - 1, rows - 1);
    let mut parts = parts.iter();
    while let Some(line) = parts.next() {
        let mut data = line.iter();
        for i in left..=right {
            if let Some(&n) = data.next() {
                result[top][i] = n;
            }
        }
        for i in (top+1)..bottom {
            if let Some(&n) = data.next() {
                result[i][right] = n;
            }
        }
        for i in (left..=right).rev() {
            if let Some(&n) = data.next() {
                result[bottom][i] = n;
            }
        }
        for i in ((top+1)..bottom).rev() {
            if let Some(&n) = data.next() {
                result[i][left] = n;
            }
        }
        top += 1;
        bottom -= 1;
        left += 1;
        right -= 1;
    }
    result
}

fn read_rows_cols_moves() -> (usize, usize, usize) {
    let a = read_usize_array();
    (a[0], a[1], a[2])
}

fn read_usize_array() -> Vec<usize> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn unassemble_assemble_01() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let unassembled = ::unassemble(&matrix);
        assert_eq!(vec![vec![1, 2, 3, 6, 9, 8, 7, 4], vec![5]], unassembled);
        assert_eq!(matrix, ::assemble(&unassembled, 3, 3));
    }

}
