use std::cmp::Ordering;

struct Date {
    day: usize,
    month: usize,
    year: usize,
}

fn main() {
    let ret_date = read_date();
    let due_date = read_date();
    println!("{}", fee(&ret_date, &due_date));
}

fn fee(ret_date: &Date, due_date: &Date) -> usize {
    match ret_date.year.cmp(&due_date.year) {
        Ordering::Less => 0,
        Ordering::Greater => 10000,
        Ordering::Equal => match ret_date.month.cmp(&due_date.month) {
            Ordering::Less => 0,
            Ordering::Greater => (ret_date.month - due_date.month) * 500,
            Ordering::Equal => match ret_date.day.cmp(&due_date.day) {
                Ordering::Less | Ordering::Equal => 0,
                Ordering::Greater => (ret_date.day - due_date.day) * 15,
            },
        },
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().into()
}

fn read_date() -> Date {
    let line: Vec<usize> = read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Date {
        day: line[0],
        month: line[1],
        year: line[2],
    }
}
