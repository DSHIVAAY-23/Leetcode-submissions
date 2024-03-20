class Solution {
public:
   int minEatingSpeed(vector<int>& piles, int h) {
        int sz = piles.size(); 
        int j = *max_element(piles.begin(), piles.end());
        // small optimization : just take max from vector if vector.size() == h 
        if(sz == h) return j;
        int i = 1;
        while(i < j){
            int temp_h = 0;
            int mid = i + ((j - i) / 2);
            for(int k = 0; k < sz; ++k)
                temp_h += ceil((double)piles[k] / mid);
            
            if (temp_h > h) i = mid + 1;
            else j = mid;
        }
        return i;
    }
};