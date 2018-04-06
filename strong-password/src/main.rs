fn main() {
    let numbers = "0123456789";
    let lower_case = "abcdefghijklmnopqrstuvwxyz";
    let upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let special_characters = "!@#$%^&*()-+";

    #[derive(Debug)]
    struct State {
        digit: u8,
        lower: u8,
        upper: u8,
        special: u8,
    }
    impl State {
        fn has_all_required_chars(&self) -> bool {
            self.digit == 0 && self.lower == 0 && self.upper == 0 && self.special == 0
        }

        fn required_chars(&self) -> u8 {
            self.digit + self.lower + self.upper + self.special
        }
    }

    let mut state = State {
        digit: 1,
        lower: 1,
        upper: 1,
        special: 1,
    };

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let count: i32 = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut chars = buf.chars().take(count as usize);

    while let Some(c) = chars.next() {
        if state.has_all_required_chars() {
            break;
        }

        if numbers.contains(c) {
            state.digit = 0;
        }
        if lower_case.contains(c) {
            state.lower = 0;
        }
        if upper_case.contains(c) {
            state.upper = 0;
        }
        if special_characters.contains(c) {
            state.special = 0;
        }
    }
    println!(
        "{}",
        if state.has_all_required_chars() && count > 5 {
            0
        } else {
            (6 - count).max(state.required_chars() as i32)
        }
    );
}
