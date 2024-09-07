impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Step 1: Filter and lowercase only the alphanumeric characters
        let filtered: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        // Step 2: Call the recursive function to check palindrome
        Solution::is_palindrome_recursive(&filtered, 0, filtered.len() as i32 - 1)
    }
    
    // Helper function for recursion
    fn is_palindrome_recursive(s: &[char], left: i32, right: i32) -> bool {
        if left >= right {
            return true; // Base case: when the pointers meet or cross, it's a palindrome
        }
        
        if s[left as usize] != s[right as usize] {
            return false; // Characters don't match, so it's not a palindrome
        }
        
        // Recursive call, moving inward
        Solution::is_palindrome_recursive(s, left + 1, right - 1)
    }
}
