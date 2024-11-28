class Solution {
public:
    int divide(int dividend, int divisor) {
        if(dividend==INT_MAX){
            if(divisor==1){
                return INT_MAX;
            }
        }
        if(dividend==INT_MIN){
            if(divisor==1){
                return INT_MIN;
            }
            else if(divisor == -1){
                return INT_MAX;
            }
        }
        return (int)dividend/divisor;
        
    }
};