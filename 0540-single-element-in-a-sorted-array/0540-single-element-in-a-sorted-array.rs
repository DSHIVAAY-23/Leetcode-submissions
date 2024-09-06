impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut output =0;
        for i in 0..nums.len(){
            output ^= nums[i];
        }
        
        return output;
    }
    
}