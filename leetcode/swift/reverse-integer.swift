// https://leetcode.com/problems/reverse-integer/

import Foundation

class Solution {
    func reverse(_ x: Int) -> Int {
        var newStr = ""
        if x < 0 {
            newStr.append("-")
        }
        
        let str = String(x)
        
        for item in str.reversed() {
            if let digit = Int(String(item)) {
                newStr.append(String(digit))
            }
        }
        
        if let answer = Int(newStr) {
            let range = Int(pow(Double(2), Double(31)))
            if answer < -range || answer > range - 1 {
                return 0
            }
            return answer
        }
        
        return 0
    }
}
