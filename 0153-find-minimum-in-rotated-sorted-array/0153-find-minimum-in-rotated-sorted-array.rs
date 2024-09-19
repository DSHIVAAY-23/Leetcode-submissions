impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
                let mut low = 0;
        let mut high = nums.len()  -1;
        let mut ans = i32::MAX;
       
        while low <= high {
            let mid = (low + high) / 2;
            
            if nums[low as usize] <= nums[high as usize]  {
                ans = std::cmp::min(ans,nums[low]);
            }
            
            if nums[low as usize] <= nums[mid as usize]{
                  ans = std::cmp::min(ans,nums[low]);

                low = mid +1;
            }   
            else{
                ans = std::cmp::min(ans,nums[mid]);
                high =mid-1;
            }
        }
            ans
    }
}