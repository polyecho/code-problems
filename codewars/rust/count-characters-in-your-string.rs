// https://www.codewars.com/kata/52efefcbcdf57161d4000091/train/rust

pub mod answer {
    use std::collections::HashMap;

    pub fn count(input: &str) -> HashMap<char, i32> {
        let mut hash_map: HashMap<char, i32> = HashMap::new();

        for item in input.chars() {
            hash_map
                .entry(item)
                .and_modify(|key| *key += 1)
                .or_insert(1);
        }

        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::answer::count;
    use std::collections::HashMap;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn test_empty_string() {
        let test_input = "";
        let expected: HashMap<char, i32> = HashMap::new();

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_two_equal_letters() {
        let test_input = "aa";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_different_letters() {
        let test_input = "aabb";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }
}
