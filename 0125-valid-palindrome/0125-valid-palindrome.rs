impl Solution {
    pub fn is_palindrome(s: String) -> bool {
      
                // Step 1: Convert to lowercase and retain only alphanumeric characters
        let filtered: String = s.chars()
            .filter(|c| c.is_alphanumeric())  // Keep only alphanumeric characters
            .map(|c| c.to_ascii_lowercase())  // Convert to lowercase
            .collect();

        // Step 2: Check if the filtered string is equal to its reverse
        let reversed: String = filtered.chars().rev().collect();

        filtered == reversed
    }
        
    
}