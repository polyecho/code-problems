// https://www.codewars.com/kata/526dbd6c8c0eb53254000110

pub mod answer {
    pub fn alphanumeric(password: &str) -> bool {
        if password.is_empty() {
            return false;
        }

        for item in password.chars() {
            if !(item.is_alphabetic() || item.is_numeric()) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::answer::alphanumeric;

    fn do_test(s: &str, expected: bool) {
        let actual = alphanumeric(s);
        assert_eq!(
            actual, expected,
            "\nInput: {s:?}\nYour result (left) did not match the expected output (right)"
        )
    }

    #[test]
    fn sample_tests() {
        do_test("hello world_", false);
        do_test("PassW0rd", true);
        do_test("     ", false);
    }
}
