impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new(); // Use a stack to keep track of characters

        for ch in s.chars() {
            // Check for 'AB' and 'CD' patterns by comparing with the top of the stack
            if let Some(&last) = stack.last() {
                if (last == 'A' && ch == 'B') || (last == 'C' && ch == 'D') {
                    stack.pop(); // Remove the matching pair from the stack
                } else {
                    stack.push(ch); // Push current character to stack
                }
            } else {
                stack.push(ch); // Stack is empty, push the character
            }
        }

        // The remaining characters in the stack form the final string
        stack.len() as i32
    }
}
