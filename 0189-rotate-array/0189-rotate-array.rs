impl Solution {
  pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        if len == 0 {
            return;
        }
        
        let k = (k as usize) % len; // Handle cases where k is larger than the array size
        let mut nums2 = vec![0; len];
        
        for i in 0..k {
            nums2[i] = nums[len - k + i];
        }
        
        for i in 0..(len - k) {
            nums2[k + i] = nums[i];
        }
        
        for i in 0..len {
            nums[i] = nums2[i];
        }
    }
}