// https://www.codewars.com/kata/52b7ed099cdc285c300001cd

pub mod answer {
    pub fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
        use std::cmp::max;

        if intervals.is_empty() {
            return 0;
        }
        let mut sorted_intervals = intervals.to_vec();
        sorted_intervals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (mut answer, mut start, mut end) = (0, sorted_intervals[0].0, sorted_intervals[0].1);

        for item in sorted_intervals.iter().skip(1) {
            if item.0 <= end {
                end = max(end, item.1);
            } else {
                answer += end - start;
                start = item.0;
                end = item.1;
            }
        }
        answer += end - start;

        answer
    }
}

#[cfg(test)]
mod sample_tests {
    use super::answer::sum_intervals;

    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }
}
