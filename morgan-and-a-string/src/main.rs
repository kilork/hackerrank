use std::borrow::Cow;

fn main() {
    let count: usize = read_line().parse().unwrap();
    for _ in 0..count {
        let s1 = read_line();
        let s2 = read_line();
        println!("{}", build_minimal_lexicographical_string(&s1, &s2));
    }
}

struct MinimalLexicographicalString<'a> {
    a: &'a [u8],
    b: &'a [u8],
    result: Vec<u8>,
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

fn compare(a: &[u8], b: &[u8]) -> bool {
    let mut left = a.to_vec();
    left.extend(b.into_iter());
    let mut right = b.to_vec();
    right.extend(a);
    left < right
}

impl<'a> MinimalLexicographicalString<'a> {
    pub fn new(s1: &'a str, s2: &'a str) -> MinimalLexicographicalString<'a> {
        MinimalLexicographicalString {
            a: s1.as_bytes(),
            b: s2.as_bytes(),
            result: Vec::with_capacity(s1.len() + s2.len()),
        }
    }

    fn solve(&'a mut self) -> Cow<'a, str> {
        let _a: Vec<String> = Tokenizer::new(self.a)
            .map(|x| String::from_utf8_lossy(x).into())
            .collect();
        let _b: Vec<String> = Tokenizer::new(self.b)
            .map(|x| String::from_utf8_lossy(x).into())
            .collect();

        let __a: Vec<&[String]> = Tokenizer::new(&_a[..]).collect();
        let __b: Vec<&[String]> = Tokenizer::new(&_b[..]).collect();
        println!("a: {:?}...", &__a[..10.min(__a.len())]);
        println!("b: {:?}...", &__b[..10.min(__b.len())]);

        let __a: Vec<String> = Tokenizer::new(&_a[..]).map(|x| x.join("")).collect();
        let __b: Vec<String> = Tokenizer::new(&_b[..]).map(|x| x.join("")).collect();
        println!("a: {:?}", &__a[..10.min(__a.len())]);
        println!("b: {:?}", &__b[..10.min(__b.len())]);

        let mut tokenizer_a = Tokenizer::new(self.a);
        let mut tokenizer_b = Tokenizer::new(self.b);
        let mut a_token = tokenizer_a.next();
        let mut a_index = 0;
        let mut b_token = tokenizer_b.next();
        let mut b_index = 0;
        while let (Some(a), Some(b)) = (a_token, b_token) {
            println!(
                "result: {:12} a: {:>12?} b: {:>12?} a_index: {} b_index: {} len: {} a: {} b: {}",
                &String::from_utf8_lossy(&self.result)[((self.result.len() as i64 -10).max(0) as usize)..],
                &_a[a_index..(a_index+10).min(_a.len())],
                &_b[b_index..(b_index+10).min(_b.len())],
                a_index, b_index, self.result.len(), String::from_utf8_lossy(a), String::from_utf8_lossy(b)
            );
            if compare(a, b) {
                self.result.extend(a);
                a_token = tokenizer_a.next();
                a_index += 1;
            } else {
                self.result.extend(b);
                b_token = tokenizer_b.next();
                b_index += 1;
            }
        }
        while let Some(a) = a_token {
            self.result.extend(a);
            a_token = tokenizer_a.next();
        }
        while let Some(b) = b_token {
            self.result.extend(b);
            b_token = tokenizer_b.next();
        }
        /*
         */
        String::from_utf8_lossy(&self.result)
    }

    fn slow_solve(&'a mut self) -> Cow<'a, str> {
        self._build(self.a, self.b).into()
    }

    fn _build(&mut self, s1: &[u8], s2: &[u8]) -> String {
        if s1.is_empty() {
            return String::from_utf8(s2.to_vec()).unwrap();
        }
        if s2.is_empty() {
            return String::from_utf8(s1.to_vec()).unwrap();
        }
        let s1_first = s1[0];
        let s2_first = s2[0];

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
        let result = format!("{}{}", result.0 as char, result.1);
        result
    }
}

fn build_minimal_lexicographical_string(s1: &str, s2: &str) -> String {
    println!("");
    MinimalLexicographicalString::new(s1, s2)
        .solve()
        .into_owned()
}

fn build_minimal_lexicographical_string_slow(s1: &str, s2: &str) -> String {
    MinimalLexicographicalString::new(s1, s2)
        .slow_solve()
        .into_owned()
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

    #[test]
    fn test_case_3() {
        let expected = include_str!("test_case_03.expected");
        let result = ::build_minimal_lexicographical_string(
            include_str!("test_case_03.s1.input"),
            include_str!("test_case_03.s2.input"),
        );

        assert_eq!(expected.len(), result.len());
        let mut from = 0;
        while from < expected.len() {
            let range = from..from + 100;
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

    #[test]
    fn test_case_4() {
        test_case_n("ABABAABAC", "ABAABABA");
    }

    #[test]
    fn test_case_5() {
        test_case_n("ABCDABCD", "ABCDABCD");
    }

    #[test]
    fn test_case_6() {
        test_case_n("ABCDABCDA", "ABCDABCDB");
    }

    #[test]
    fn test_case_7() {
        test_case_n(
            "BBBBBBBBAKBDFAAAFMMMAKAAZAKJKMNZZZ",
            "BABAKBDFABAFBKBANZJZZA",
        );
    }

    fn test_case_n(a: &str, b: &str) {
        assert_eq!(
            ::build_minimal_lexicographical_string_slow(a, b),
            ::build_minimal_lexicographical_string(a, b)
        );
    }

    #[test]
    fn tokenizer_1() {
        let mut tokenizer = ::Tokenizer::new(b"BBBBBBBBAKBDFAAAFMMMAKAAZAKJKMNZZZ");
        assert_eq!(Some(&b"BBBBBBBBA"[..]), tokenizer.next());
        assert_eq!(Some(&b"KBDFAAAF"[..]), tokenizer.next());
        assert_eq!(Some(&b"MMMAKAA"[..]), tokenizer.next());
        assert_eq!(Some(&b"ZAKJKMN"[..]), tokenizer.next());
        assert_eq!(Some(&b"ZZZ"[..]), tokenizer.next());
        assert_eq!(None, tokenizer.next());

        tokenizer = ::Tokenizer::new(b"BABAKBDFABAFBKBANZJZZA");
        assert_eq!(Some(&b"BA"[..]), tokenizer.next());
        assert_eq!(Some(&b"BA"[..]), tokenizer.next());
        assert_eq!(Some(&b"KBDFABAFB"[..]), tokenizer.next());
        assert_eq!(Some(&b"KBA"[..]), tokenizer.next());
        assert_eq!(Some(&b"N"[..]), tokenizer.next());
        assert_eq!(Some(&b"ZJ"[..]), tokenizer.next());
        assert_eq!(Some(&b"ZZA"[..]), tokenizer.next());
        assert_eq!(None, tokenizer.next());

        assert!(&b"ZZ"[..] < &b"ZZZ"[..]);
        assert!(&b"ZZZZ"[..] > &b"ZZZ"[..]);
        assert!(&b"ZZA"[..] < &b"ZZZ"[..]);
    }
}
//                      ABCDABCDA ABCDABCDB                          ABCDABCDA ABCDABCDB
// A                     BCDABCDA ABCDABCDB     A                     BCDABCDA ABCDABCDB
// AA                    BCDABCDA  BCDABCDB     AA                    BCDABCDA  BCDABCDB
// AAB                    CDABCDA  BCDABCDB     AAB                   BCDABCDA   CDABCDB
// AABB                   CDABCDA   CDABCDB     AABB                   CDABCDA   CDABCDB
// AABBC                   DABCDA   CDABCDB     AABBC                  CDABCDA    DABCDB
// AABBCC                  DABCDA    DABCDB     AABBCC                  DABCDA    DABCDB
// AABBCCD                  ABCDA    DABCDB     AABBCCD                 DABCDA     ABCDB
// AABBCCDA                  BCDA    DABCDB     AABBCCDA                DABCDA      BCDB
// AABBCCDAB                  CDA    DABCDB     AABBCCDAB               DABCDA       CDB
// AABBCCDABC                  DA    DABCDB     AABBCCDABC              DABCDA        DB
// AABBCCDABCD                  A    DABCDB     AABBCCDABCD              ABCDA        DB
// AABBCCDABCDA                      DABCDB     AABBCCDABCDA              BCDA        DB
//                                              AABBCCDABCDAB              CDA        DB
//                                              AABBCCDABCDABC              DA        DB
//                                              AABBCCDABCDABCD              A        DB
//                                              AABBCCDABCDABCDA                      DB
// AABBCCDABCDADABCDB                           AABBCCDABCDABCDADB

// AABBCCDABCDADABCDB
// AABBCCDABCDABCDADB
