// https://leetcode.com/problems/fizz-buzz

class Solution {
    func fizzBuzz(_ n: Int) -> [String] {
        var answer: [String] = []
        
        for i in 1...n {
            if i % 3 != 0 && i % 5 != 0 {
                answer.append("\(i)")
                continue
            }
            
            var appendingMsg = ""
            if i % 3 == 0 {
                appendingMsg.append("Fizz")
            }
            if i % 5 == 0 {
                appendingMsg.append("Buzz")
            }
            
            answer.append(appendingMsg)
        }
        
        return answer
    }
}
