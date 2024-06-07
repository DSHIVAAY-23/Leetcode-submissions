impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut doge = 0;
        for i in 0..nums.len() {
            doge = doge^nums[i];           
}
        return doge as i32;
        
    }
}