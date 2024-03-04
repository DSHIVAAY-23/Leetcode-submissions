
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.len();
        let mut m = HashMap::new();
        for i in 0..l {
            let t = target-nums[i];
            if let Some(x) = m.get(&t) {
                return vec![*x, i as i32];
            } else {
                m.insert(nums[i], i as i32);
            }
        }
        vec![]
    }
}