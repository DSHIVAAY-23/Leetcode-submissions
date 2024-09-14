impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut el = nums[0];
      let mut count = 0;
     for &i in &nums {
         
         if count == 0{
             el = i;
         }
         if i == el {
             count +=1;
         }
         else{
             count -=1;
         }
          
        
         
        }
     el
    }
}