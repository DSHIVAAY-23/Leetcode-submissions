impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();  // Sort the array first
        let n = nums.len();
        let mut ans: Vec<Vec<i32>> = Vec::new();
      
        for i in 0..n {
            // Skip duplicates for the first element of the triplet
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            let mut j = i + 1;
            let mut k = n - 1;
            
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                
                if sum < 0 {
                    j += 1;  // Move j right to increase the sum
                } else if sum > 0 {
                    k -= 1;  // Move k left to decrease the sum
                } else {
                    ans.push(vec![nums[i], nums[j], nums[k]]);  // Found a valid triplet
                    
                    // Move both pointers to the next unique elements
                    j += 1;
                    k -= 1;
                    
                    // Skip duplicates for the second element
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    // Skip duplicates for the third element
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
            }
        }
        
        ans
    }
}