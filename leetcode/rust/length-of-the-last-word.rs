// https://leetcode.com/problems/length-of-last-word/

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut splitted_s: Vec<&str> = s.split(" ").collect();
        splitted_s.reverse();

        for item in splitted_s {
            if item != "" && item != " " {
                return item.len() as i32;
            }
        }

        0
    }
}
