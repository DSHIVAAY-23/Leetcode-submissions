class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int n = nums.size();
        int x = n/2;
        int temp ;
        int cnt =0;
        for(int i = 0 ; i < n ; i++){
            if(cnt == 0){
                cnt = 1;
                temp = nums[i];
            }else if(temp == nums[i]){
                cnt++;
            }else {
                cnt--;
            }
        }
        int cnt1 = 0;
        for(int i = 0; i<n; i++){
            if(nums[i]==temp)cnt1++;
        }
       if (cnt1 > (n / 2)) return temp;
    return -1;
    }
};