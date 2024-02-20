// https://leetcode.com/problems/find-the-difference/

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();

        s_vec.sort();
        t_vec.sort();

        for (index, t_item) in t_vec.into_iter().enumerate() {
            if index >= s_vec.len() || t_item != s_vec[index] {
                return t_item;
            }
        }

        // Fallback.
        '_'
    }
}
