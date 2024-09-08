impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
       
        if n==0{
            return false;
        }
        if n>0 && n &(n-1) ==0{
            return true;
        }
        else{
            false
        }
    }
}