class Solution {
public:
    void rotate(vector<int>& arr, int k) {
        int n = arr.size();
    if (n == 0)
    return;
  k = k % n;
  if(k == 0) return;
  int temp[k];
  for (int i = n-k; i < n; i++)
  {
    temp[i-n+k] = arr[i];
  }
  for (int i = n-1 ; i >= k ; i--)
  {
    arr[i] = arr[i - k];
  }
  for (int i = 0; i < k; i++)
  {
    arr[i] = temp[i];
  }
    }
};