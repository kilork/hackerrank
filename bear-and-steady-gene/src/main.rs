use std::collections::HashMap;
use std::ops::SubAssign;

fn main() {
    let _n: usize = read_line().parse().unwrap();
    let gene = read_line();
    let min_edit = bear_and_steady_gene(&gene);
    println!("{}", min_edit);
}

struct GeneFixer<'a> {
    gene: &'a [u8],
    counter: GeneCounter,
    calls_count: usize,
    runs_count: usize,
    cache_hits: usize,
    cache: HashMap<CacheKey, Option<GeneRange>>,
}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    start: usize,
    a: i32,
    c: i32,
    t: i32,
    g: i32,
}

impl<'a> From<(usize, &'a GeneCounter)> for CacheKey {
    fn from(key: (usize, &'a GeneCounter)) -> CacheKey {
        CacheKey {
            start: key.0,
            a: key.1.a,
            c: key.1.c,
            t: key.1.t,
            g: key.1.g,
        }
    }
}

impl<'a> From<&'a str> for GeneFixer<'a> {
    fn from(gene: &'a str) -> GeneFixer<'a> {
        GeneFixer::from(gene.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for GeneFixer<'a> {
    fn from(gene: &'a [u8]) -> GeneFixer<'a> {
        let counter = GeneCounter::from(gene);
        GeneFixer {
            gene,
            counter,
            calls_count: 0,
            runs_count: 0,
            cache_hits: 0,
            cache: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct GeneCounter {
    a: i32,
    c: i32,
    t: i32,
    g: i32,
}

impl<'a> From<&'a [u8]> for GeneCounter {
    fn from(val: &[u8]) -> GeneCounter {
        let a = val.iter().filter(|&&x| x == b'A').count() as i32;
        let c = val.iter().filter(|&&x| x == b'C').count() as i32;
        let t = val.iter().filter(|&&x| x == b'T').count() as i32;
        let g = val.iter().filter(|&&x| x == b'G').count() as i32;

        let mut gene_counter = GeneCounter { a, c, t, g };

        let len = val.len() as i32 / 4;
        gene_counter -= len;

        gene_counter
    }
}

impl SubAssign<i32> for GeneCounter {
    fn sub_assign(&mut self, other: i32) {
        self.a -= other;
        self.c -= other;
        self.t -= other;
        self.g -= other;
    }
}

impl PartialEq<i32> for GeneCounter {
    fn eq(&self, other: &i32) -> bool {
        let other = *other;
        self.a == other && self.c == other && self.t == other && self.g == other
    }
}

impl GeneCounter {
    fn is_done(&self) -> bool {
        self.a <= 0 && self.c <= 0 && self.t <= 0 && self.g <= 0
    }

    fn theoretical_min(&self) -> usize {
        let mut theoretical_min = 0;
        if self.a > 0 {
            theoretical_min += self.a;
        }
        if self.c > 0 {
            theoretical_min += self.c;
        }
        if self.t > 0 {
            theoretical_min += self.t;
        }
        if self.g > 0 {
            theoretical_min += self.g;
        }
        theoretical_min as usize
    }

    fn try_decrease(&self, value: u8) -> Option<GeneCounter> {
        match value {
            b'A' if self.a > 0 => Some(GeneCounter {
                a: self.a - 1,
                ..*self
            }),
            b'C' if self.c > 0 => Some(GeneCounter {
                c: self.c - 1,
                ..*self
            }),
            b'T' if self.t > 0 => Some(GeneCounter {
                t: self.t - 1,
                ..*self
            }),
            b'G' if self.g > 0 => Some(GeneCounter {
                g: self.g - 1,
                ..*self
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct GeneRange(usize, usize);

impl<'a> GeneFixer<'a> {
    pub fn fix(&mut self) -> usize {
        let counter = self.counter.clone();
        let result = self.solve(0, &counter).unwrap();
        if option_env!("DEBUG") == Some("yes") {
            println!("result: {:?}", result);
            println!("counter: {:?}", counter);
            println!("calls: {}, runs: {}, cache hits: {}, cache size: {}", self.calls_count, self.runs_count, self.cache_hits, self.cache.len());
        }
        result.1
    }

    pub fn scan(&mut self) -> usize {
        let counter = self.counter.clone();
        let theoretical_min = counter.theoretical_min();
        if theoretical_min == 0 {
            return 0;
        }
        let mut min_len = self.gene.len();
        for start in 0..self.gene.len() - theoretical_min {
            if start % 1000 == 0 {
                println!("{}", start);
            }
            let mut counter = counter.clone();
            let mut count = 0;
            let mut len = 0;
            let mut seeker = self.gene.iter().skip(start);
            while let Some(c) = seeker.next() {
                len += 1;
                if let Some(new_counter) = counter.try_decrease(*c) {
                    counter = new_counter;
                    count += 1;
                    if count == theoretical_min {
                        if len < min_len {
                            min_len = len;
                            if min_len == theoretical_min {
                                return min_len;
                            }
                        }
                        break;
                    }
                }
            }
        }
        min_len
    }

    fn solve(&mut self, start: usize, gene_counter: &GeneCounter) -> Option<GeneRange> {
        self.calls_count += 1;
        if gene_counter.is_done() {
            return Some(GeneRange(start, 0));
        }
        if start == self.gene.len() {
            return None;
        }
        let cache_key = CacheKey::from((start, gene_counter));
        if let Some(solution) = self.cache.get(&cache_key) {
            self.cache_hits += 1;
            return solution.clone();
        }
        self.runs_count += 1;
        let solution = {
            let tail_solution = self.solve(start + 1, gene_counter);
            if let Some(remaining_counter) = gene_counter.try_decrease(self.gene[start]) {
                if let Some(GeneRange(remaining_start, remaining_len)) =
                    self.solve(start + 1, &remaining_counter)
                {
                    let remaining_solution = remaining_start - start + remaining_len;
                    if let Some(GeneRange(tail_start, tail_len)) = tail_solution {
                        if remaining_solution < tail_len {
                            Some(GeneRange(start, remaining_solution))
                        } else {
                            Some(GeneRange(tail_start, tail_len))
                        }
                    } else {
                        Some(GeneRange(start, remaining_solution))
                    }
                } else {
                    tail_solution
                }
            } else {
                tail_solution
            }
        };
        self.cache.insert(cache_key, solution.clone());
        solution
    }
}

fn bear_and_steady_gene(gene: &str) -> usize {
    // GeneFixer::from(gene).fix()
    GeneFixer::from(gene).scan()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_0() {
        assert_eq!(0, ::bear_and_steady_gene("CATG"))
    }

    #[test]
    fn test_case_1() {
        assert_eq!(3, ::bear_and_steady_gene("GGGG"))
    }

    #[test]
    fn test_case_2() {
        assert_eq!(5, ::bear_and_steady_gene("GAAATAAA"))
    }

    #[test]
    fn test_case_3() {
        assert_eq!(5, ::bear_and_steady_gene("TGATGCCGTCCCCTCAACTTGAGTGCTCCTAATGCGTTGC"))
    }
}
