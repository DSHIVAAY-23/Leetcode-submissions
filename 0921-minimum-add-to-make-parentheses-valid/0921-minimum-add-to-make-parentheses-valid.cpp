class Solution {
public:
    int minAddToMakeValid(string s) {
        int count_1 = 0;
        int count_2 = 0;
        int output = 0;
       for (int i = 0; i<s.size();i++) {
           if (s[i] == '('){
               count_1++;
           }
           else {
               if (count_1>0){
               count_1--;
               }
               else{
                   count_2++;
               }
           }
           
       }
      return  output = count_2 + count_1;
        
    }
};