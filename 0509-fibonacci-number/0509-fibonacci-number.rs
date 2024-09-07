impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut output1 =Self::fib(n-1);
        let mut output2=Self::fib(n-2);
        let mut output3 = output1 + output2;
        return output3
    }
}