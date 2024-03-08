class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int n = nums.size();
        int x = n/2;
        int temp = 0;

        for(int i = 0;i<n;i++){
            int cnt = 0;
            for(int j = 0;j<n;j++){
                if(nums[i]==nums[j]){
                    cnt++;
                }
            }
            
   
                  if (cnt > x)
            return nums[i];
		
        }
        return -1;
        
    }
};