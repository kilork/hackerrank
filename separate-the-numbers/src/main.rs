fn main() {
    let count: usize = read_line().parse().unwrap();
    for _ in 0..count {
        let nums = read_numbers();
        let answer = match seek_perfect_numbers(&nums) {
            Some(perfect_number) => format!("YES {}", perfect_number),
            None => "NO".to_string(),
        };
        println!("{}", answer)
    }
}

fn seek_perfect_numbers(nums: &[u64]) -> Option<u64> {
    let mut length = 1;
    if let Some(&0) = nums.get(0) {
        return None;
    }
    while length <= nums.len() / 2 {
        let mut result = Vec::new();

        let mut number = 0;
        for i in 0..length {
            number = number * 10 + nums[i];
        }
        let perfect_number = number;
        let mut pos = length;
        'outer: loop {
            result.push(number);
            number += 1;
            let snumber: Vec<u64> = number
                .to_string()
                .chars()
                .map(|x| x as u64 - '0' as u64)
                .collect();
            let upper_bound = pos + snumber.len();
            if upper_bound > nums.len() {
                break;
            }
            for i in pos..upper_bound {
                if nums[i] != snumber[i - pos] {
                    break 'outer;
                }
            }
            pos = upper_bound;
            if pos == nums.len() {
                return Some(perfect_number);
            }
        }
        length += 1;
    }
    None
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn read_numbers() -> Vec<u64> {
    read_line().chars().map(|x| x as u64 - '0' as u64).collect()
}
