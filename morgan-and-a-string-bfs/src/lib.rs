#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Solution {
    a: usize,
    b: usize,
    is_a: bool,
}

use std::fmt;

impl fmt::Debug for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ a: {}, b: {} }}", self.a, self.b)
    }
}

impl Solution {
    fn next_empty(&self, msf: &MinimalSequenceFinder, next: &mut Vec<Solution>) {
        let (next_a, next_b) = if self.is_a {
            (self.a + 1, self.b)
        } else {
            (self.a, self.b + 1)
        };
        if next_a < msf.a.len() {
            next.push(Solution {
                a: next_a,
                b: next_b,
                is_a: true,
            });
        }
        if next_b < msf.b.len() {
            next.push(Solution {
                a: next_a,
                b: next_b,
                is_a: false,
            });
        }
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
        let a: Vec<&[u8]> = Tokenizer::new(self.a).collect();
        let b: Vec<&[u8]> = Tokenizer::new(self.b).collect();

        let mut solutions: Vec<Solution> = vec![
            Solution {
                a: 0,
                b: 0,
                is_a: true,
            },
            Solution {
                a: 0,
                b: 0,
                is_a: false,
            },
        ];
        loop {
            let mut min = std::u8::MAX;

            for i in 0..solutions.len() {
                let s = &solutions[i];
                let current = if s.is_a {
                    self.a[s.a]
                } else {
                    self.b[s.b]
                };
                if current < min {
                    min = current;
                }
            }

            self.result.push(min);

            let mut next_solutions = vec![];
            for i in 0..solutions.len() {
                let s = &solutions[i];
                let current = if s.is_a {
                    self.a[s.a]
                } else {
                    self.b[s.b]
                };
                if current == min {
                    s.next_empty(&self, &mut next_solutions);
                }
            }
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

struct Tokenizer<'a, T: 'a> {
    data: &'a [T],
    index: usize,
}

impl<'a, T> Tokenizer<'a, T> {
    fn new(data: &[T]) -> Tokenizer<T> {
        Tokenizer { data, index: 0 }
    }
}

impl<'a, T: 'a> Iterator for Tokenizer<'a, T>
where
    T: PartialEq + PartialOrd,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.data.len() {
            return None;
        }
        let start = self.index;
        let first = &self.data[self.index];
        while self.index < self.data.len() && *first == self.data[self.index] {
            self.index += 1;
        }
        while self.index < self.data.len() && *first > self.data[self.index] {
            self.index += 1;
        }
        Some(&self.data[start..self.index])
    }
}

pub fn minimal_sequence_finder<'a>(a: &'a str, b: &'a str) -> String {
    String::from_utf8_lossy(MinimalSequenceFinder::new(a, b).solve()).into_owned()
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