fn main() {
    let count: usize = read_line().parse().unwrap();
    for _ in 0..count {
        let s1 = read_line();
        let s2 = read_line();
        println!("{}", build_minimal_lexicographical_string(&s1, &s2));
    }
}

struct MinimalLexicographicalString<'a> {
    s1: &'a str,
    s2: &'a str,
    s1_len: usize,
    cache: Vec<Option<String>>,
}

impl<'a> MinimalLexicographicalString<'a> {
    pub fn new(s1: &'a str, s2: &'a str) -> MinimalLexicographicalString<'a> {
        MinimalLexicographicalString {
            s1,
            s2,
            s1_len: s1.len(),
            cache: vec![None; (s1.len() + 1) * (s2.len() + 1)],
        }
    }

    fn solve(&mut self) -> String {
        self._build(self.s1, self.s2)
    }

    fn _build(&mut self, s1: &str, s2: &str) -> String {
        if s1.is_empty() {
            return s2.into();
        }
        if s2.is_empty() {
            return s1.into();
        }
        if let Some(ref result) = self.cache[self.s1_len * s1.len() + s2.len()] {
            return result.clone();
        }
        let s1_first = s1.chars().next().unwrap();
        let s2_first = s2.chars().next().unwrap();

        let result = if s1_first < s2_first {
            (s1_first, self._build(&s1[1..], s2))
        } else if s2_first < s1_first {
            (s2_first, self._build(s1, &s2[1..]))
        } else {
            let solution_s1 = self._build(&s1[1..], s2);
            let solution_s2 = self._build(s1, &s2[1..]);
            if solution_s1 < solution_s2 {
                (s1_first, solution_s1)
            } else {
                (s2_first, solution_s2)
            }
        };
        let result = format!("{}{}", result.0, result.1);
        self.cache[self.s1_len * s1.len() + s2.len()] = Some(result.clone());
        result
    }
}

fn build_minimal_lexicographical_string(s1: &str, s2: &str) -> String {
    MinimalLexicographicalString::new(s1, s2).solve()
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
        assert_eq!(
            "DAJACKNIEL",
            ::build_minimal_lexicographical_string("JACK", "DANIEL")
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            "AABABACABACABA",
            ::build_minimal_lexicographical_string("ABACABA", "ABACABA")
        );
    }
}
