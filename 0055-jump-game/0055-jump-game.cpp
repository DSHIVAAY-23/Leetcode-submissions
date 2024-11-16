class Solution {
public:
    bool canJump(vector<int>& nums) {
        int n = nums.size();
        int j = 0;
        
        for(int i =0;i<n;i++){
            if(i<=j){
                j = max(j,i+nums[i]);
            }
            else{
                return false;   
            }
            
        }
return true;        
    }
};