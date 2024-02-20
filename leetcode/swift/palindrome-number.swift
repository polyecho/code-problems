// https://leetcode.com/problems/palindrome-number/

import Foundation

class Solution {
    func isPalindrome(_ x: Int) -> Bool {
        let string = String(x)
        let characterArray = Array(string)
        let count = characterArray.count
        
        for index in 0..<count/2 {
            if characterArray[index] != characterArray[count - 1 - index] {
                return false
            }
        }
        
        return true;
    }
}
