class Solution {
public:
    int fib(int n) {
        if(n==0)return 0;
        if (n==1) return 1;
        int s1 = fib(n-1);
        int s2 = fib(n-2);
        return s1+s2;
        
    }
};