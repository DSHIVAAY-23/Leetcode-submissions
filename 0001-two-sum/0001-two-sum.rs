impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let n = nums.len();
        for i in 0..n {
            for j in i+1..n{ 
            if nums[i]+nums[j] == target{
                ans.push(i as i32);
                ans.push(j as i32);
                return ans;
                }
            }
        }
        ans
    }
}