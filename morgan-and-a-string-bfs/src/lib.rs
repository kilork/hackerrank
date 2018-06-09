pub fn minimal_sequence_finder<'a>(a: &'a str, b: &'a str) -> String {
    String::from_utf8_lossy(MinimalSequenceFinder::new(a, b).solve()).into_owned()
}

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
    fn next_empty(&self, a: &[&[u8]], b: &[&[u8]], next: &mut Vec<Solution>) {
        let (next_a, next_b) = if self.is_a {
            (self.a + 1, self.b)
        } else {
            (self.a, self.b + 1)
        };
        if next_a < a.len() {
            next.push(Solution {
                a: next_a,
                b: next_b,
                is_a: true,
            });
        }
        if next_b < b.len() {
            next.push(Solution {
                a: next_a,
                b: next_b,
                is_a: false,
            });
        }
    }

    fn token<'a>(&self, a: &'a [&[u8]], b: &'a [&[u8]]) -> &'a [u8] {
        if self.is_a {
            a[self.a]
        } else {
            b[self.b]
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
            let token = |x: &Solution| x.token(&a, &b);
            let mut min = token(&solutions[0]);

            for i in 1..solutions.len() {
                let current = token(&solutions[i]);
                if less(current, min) {
                    min = current;
                }
            }

            self.result.extend(min);

            let mut next_solutions = vec![];

            for i in 0..solutions.len() {
                let s = &solutions[i];
                let current = token(s);
                if current == min {
                    s.next_empty(&a, &b, &mut next_solutions);
                }
            }

            if next_solutions.is_empty() {
                break;
            }

            next_solutions.sort();
            next_solutions.dedup();

            solutions = next_solutions;
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

fn less(a: &[u8], b: &[u8]) -> bool {
    if a.len() < b.len() {
        if a < b {
            a < &b[..a.len()]
        } else {
            false
        }
    } else if b.len() < a.len() {
        if a > b {
            !(&a[..b.len()] > b)
        } else {
            true
        }
    } else {
        a < b
    }
}

#[cfg(test)]
mod tests {
    fn less(a: &str, b: &str) -> bool {
        ::less(a.as_bytes(), b.as_bytes())
    }

    #[test]
    fn less_is_ok() {
        assert_eq!(true, less("A", "B"));
        assert_eq!(false, less("B", "A"));
        assert_eq!(false, less("B", "B"));
        assert_eq!(false, less("A", "A"));
        assert_eq!(false, less("A", "AA"));
        assert_eq!(true, less("AA", "A"));
        assert_eq!(false, less("B", "BAA"));
        assert_eq!(true, less("BAA", "B"));
    }

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
