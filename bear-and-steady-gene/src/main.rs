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
}

impl<'a> From<&'a str> for GeneFixer<'a> {
    fn from(gene: &'a str) -> GeneFixer<'a> {
        GeneFixer::from(gene.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for GeneFixer<'a> {
    fn from(gene: &'a [u8]) -> GeneFixer<'a> {
        let counter = GeneCounter::from(gene);
        GeneFixer { gene, counter }
    }
}

impl<'a> GeneFixer<'a> {
    pub fn solve(&mut self) -> usize {
        let theoretical_min = self.counter.count_positive();

        if theoretical_min == 0 {
            return 0;
        }

        let (mut left, mut right) = (theoretical_min, self.gene.len());

        if self.is_solution(left) {
            return left;
        } else {
            loop {
                let mid = (left + right + 1) / 2;

                if self.is_solution(mid) {
                    right = mid;

                    if right == left + 1 {
                        return right;
                    }
                } else {
                    left = mid;
                }
            }
        }
    }

    fn is_solution(&mut self, len: usize) -> bool {
        let first_set = &self.gene[..len];

        let mut last_counter = GeneCounter::default();

        last_counter.add_items(first_set);

        if last_counter.is_superset(&self.counter) {
            return true;
        }

        for i in 1..self.gene.len() - len + 1 {
            last_counter.sub_item(self.gene[i - 1]);
            last_counter.add_item(self.gene[i + len - 1]);

            if last_counter.is_superset(&self.counter) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Default)]
struct GeneCounter {
    a: i32,
    c: i32,
    t: i32,
    g: i32,
}

impl<'a> From<&'a [u8]> for GeneCounter {
    fn from(val: &[u8]) -> GeneCounter {
        let mut gene_counter = GeneCounter::default();
        gene_counter.add_items(val);

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

impl GeneCounter {
    fn add_items(&mut self, data: &[u8]) {
        data.iter().fold(self, |acc, x| {
            acc.add_item(*x);
            acc
        });
    }

    fn add_item(&mut self, item: u8) {
        match item {
            b'A' => self.a += 1,
            b'C' => self.c += 1,
            b'T' => self.t += 1,
            b'G' => self.g += 1,
            _ => (),
        }
    }

    fn sub_item(&mut self, item: u8) {
        match item {
            b'A' => self.a -= 1,
            b'C' => self.c -= 1,
            b'T' => self.t -= 1,
            b'G' => self.g -= 1,
            _ => (),
        }
    }

    fn is_superset(&self, other: &GeneCounter) -> bool {
        ((self.a >= other.a) || (other.a <= 0)) && ((self.c >= other.c) || (other.c <= 0))
            && ((self.t >= other.t) || (other.t <= 0))
            && ((self.g >= other.g) || (other.g <= 0))
    }

    fn count_positive(&self) -> usize {
        let mut count = 0;
        if self.a > 0 {
            count += self.a;
        }
        if self.c > 0 {
            count += self.c;
        }
        if self.t > 0 {
            count += self.t;
        }
        if self.g > 0 {
            count += self.g;
        }
        count as usize
    }
}

fn bear_and_steady_gene(gene: &str) -> usize {
    GeneFixer::from(gene).solve()
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
        assert_eq!(
            5,
            ::bear_and_steady_gene("TGATGCCGTCCCCTCAACTTGAGTGCTCCTAATGCGTTGC")
        )
    }
}
