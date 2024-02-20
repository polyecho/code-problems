// https://www.codewars.com/kata/514b92a657cdc65150000006/

pub mod answer {
    pub fn solution(num: i32) -> i32 {
        if num < 0 {
            return 0;
        }

        let range: Vec<i32> = (0..num)
            .filter(|item| (item % 3 == 0 || item % 5 == 0) && item < &num)
            .collect();

        let mut return_value = 0;

        for item in range {
            return_value += item;
        }

        return_value
    }
}

mod tests {
    use super::answer::solution;

    #[test]
    fn sample_tests() {
        // assertion(expected, input);
        assertion(23, 10);
        assertion(33, 11);
        assertion(225, 33);
        assertion(8, 6);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(0, 0);
        assertion(0, -203);
        assertion(25719750, 10500);
    }

    fn assertion(expected: i32, input: i32) {
        let actual = solution(input);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n",
            expected,
            actual,
            input
        );
    }
}
