impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut n =nums.len();
        let mut count = 0;
        for i in 0..n {
         let mut sum =0;

            for j in i..n{
            sum += nums[j as usize];
            if sum ==k {
                count += 1; 
                
            }
            }
            
            
        }
        count
        
    }
}