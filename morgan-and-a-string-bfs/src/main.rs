fn main() {
    let count: usize = read_line().parse().unwrap();

    for _ in 0..count {
        let a = read_line();
        let b = read_line();
        println!("{}", minimal_sequence_finder(&a, &b));
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Solution {
    data: u8,
    a: usize,
    b: usize,
    is_a: bool,
}

use std::fmt;

impl fmt::Debug for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ data: {}, a: {}, b: {} }}",
            self.data as char, self.a, self.b
        )
    }
}

impl Solution {
    fn append_to(&self, result: &mut Vec<u8>) {
        result.push(self.data);
    }

    fn next_empty(&self, msf: &MinimalSequenceFinder) -> Vec<Solution> {
        let mut next = vec![];
        let (next_a, next_b) = if self.is_a {
            (self.a + 1, self.b)
        } else {
            (self.a, self.b + 1)
        };
        if next_a < msf.a.len() {
            next.push(Solution {
                data: msf.a[next_a],
                a: next_a,
                b: next_b,
                is_a: true,
            });
        }
        if next_b < msf.b.len() {
            next.push(Solution {
                data: msf.b[next_b],
                a: next_a,
                b: next_b,
                is_a: false,
            });
        }
        next
    }
}

struct MinimalSequenceFinder<'a> {
    a: &'a [u8],
    b: &'a [u8],
    result: Vec<u8>,
}

impl<'a> MinimalSequenceFinder<'a> {
    fn new(a: &'a str, b: &'a str) -> MinimalSequenceFinder<'a> {
        MinimalSequenceFinder {
            a: a.as_bytes(),
            b: b.as_bytes(),
            result: vec![],
        }
    }

    fn solve(&mut self) -> &[u8] {
        if !self.result.is_empty() {
            return &self.result;
        }

        let mut solutions: Vec<Solution> = vec![
            Solution {
                data: self.a[0],
                a: 0,
                b: 0,
                is_a: true,
            },
            Solution {
                data: self.b[0],
                a: 0,
                b: 0,
                is_a: false,
            },
        ];
        loop {
            let mut min = std::u8::MAX;
            let mut min_solution = None;

            for i in 0..solutions.len() {
                let current = solutions[i].data;
                if current < min {
                    min = current;
                    min_solution = Some(i);
                }
            }

            let index = min_solution.unwrap();
            solutions[index].append_to(&mut self.result);

            let mut next_solutions: Vec<Solution> = solutions
                .iter()
                .filter_map(|x| {
                    if x.data == min {
                        Some(x.next_empty(self))
                    } else {
                        None
                    }
                })
                .flat_map(|x| x)
                .collect();
            let before_len = next_solutions.len();
            next_solutions.sort();
            next_solutions.dedup();
            let after_len = next_solutions.len();
            if before_len != after_len {
                println!("before: {} after: {}", before_len, after_len);
            }

            solutions.clear();
            solutions.extend(next_solutions);

            if solutions.is_empty() {
                break;
            }
        }

        &self.result
    }
}

fn minimal_sequence_finder<'a>(a: &'a str, b: &'a str) -> String {
    String::from_utf8_lossy(MinimalSequenceFinder::new(a, b).solve()).into_owned()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        assert_eq!("DAJACKNIEL", ::minimal_sequence_finder("JACK", "DANIEL"));
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            "AABABACABACABA",
            ::minimal_sequence_finder("ABACABA", "ABACABA")
        );
    }

    #[test]
    fn test_case_3() {
        let expected = include_str!("test_case_03.expected");
        let result = ::minimal_sequence_finder(
            include_str!("test_case_03.s1.input"),
            include_str!("test_case_03.s2.input"),
        );

        assert_eq!(expected.len(), result.len());
        let mut from = 0;
        while from < expected.len() {
            let range = from..(from + 100).min(expected.len());
            assert_eq!(
                expected[range.clone()],
                result[range.clone()],
                "range {:?} failed, total len: {}",
                range,
                expected.len()
            );
            from += 100;
        }
        assert_eq!(expected, result);
    }

}
