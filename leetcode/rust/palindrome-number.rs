// https://leetcode.com/problems/palindrome-number/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string = x.to_string();
        let string_length = string.len() as f64;

        let char_vec: Vec<char> = string.chars().collect();
        let range: f64 = (string_length / 2_f64).ceil();

        for i in 0..(range as usize) {
            if char_vec[i] != char_vec[string_length as usize - 1 - i] {
                return false;
            }
        }

        true
    }
}
