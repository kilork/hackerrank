// good to read:
//https://en.wikipedia.org/wiki/Edit_distance

fn main() {
    let s1 = read_line();
    let s2 = read_line();
    println!("{}", common_child_length(&s1, &s2));
}

struct CommonChildFinder<'a> {
    s1: &'a [u8],
    s2: &'a [u8],
    s1_len: usize,
    s2_len: usize,
    solutions: Vec<Option<usize>>,
}

impl<'a> CommonChildFinder<'a> {
    pub fn new(s1: &'a str, s2: &'a str) -> CommonChildFinder<'a> {
        CommonChildFinder {
            s1: s1.as_bytes(),
            s2: s2.as_bytes(),
            s1_len: s1.len(),
            s2_len: s2.len(),
            solutions: vec![None; s1.len() * s2.len()],
        }
    }

    pub fn solve(&mut self) -> usize {
        self.solve_bytes(0, 0)
    }

    fn solve_bytes(&mut self, s1_from: usize, s2_from: usize) -> usize {
        if s1_from == self.s1_len || s2_from == self.s2_len {
            return 0;
        }

        if let Some(solution) = self.find_solution(s1_from, s2_from) {
            return solution;
        }

        let c = self.s1[s1_from];
        let mut solution = self.solve_bytes(s1_from + 1, s2_from);
        if let Some(index) = self.s2.iter().skip(s2_from).position(|&x| c == x) {
            if self.s2_len - s2_from - index >= solution {
                solution = (1 + self.solve_bytes(s1_from + 1, s2_from + index + 1)).max(solution)
            }
        }
        self.write_solution(s1_from, s2_from, solution);
        solution
    }

    #[inline]
    fn write_solution(&mut self, s1_from: usize, s2_from: usize, solution: usize) {
        self.solutions[s2_from * self.s2_len + s1_from] = Some(solution);
    }

    #[inline]
    fn find_solution(&self, s1_from: usize, s2_from: usize) -> Option<usize> {
        self.solutions[s2_from * self.s2_len + s1_from]
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_string()
}

fn common_child_length(s1: &str, s2: &str) -> usize {
    CommonChildFinder::new(s1, s2).solve()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        assert_eq!(3, ::common_child_length("ABCD", "ABDC"));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(0, ::common_child_length("AA", "BB"));
    }

    #[test]
    fn test_case_3() {
        assert_eq!(2, ::common_child_length("HARRY", "SALLY"));
    }

    #[test]
    fn test_case_4() {
        assert_eq!(3, ::common_child_length("SHINCHAN", "NOHARAAA"));
    }

    #[test]
    fn test_case_5() {
        assert_eq!(2, ::common_child_length("ABCDEF", "FBDAMN"));
    }

    #[test]
    fn test_case_6() {
        assert_eq!(2, ::common_child_length("OUDFRMYMAW", "AWHYFCCMQX"));
    }

}
