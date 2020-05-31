// C version has some obsurd inputs to the function without description = =
// fuck leetcode
//
// so I chose C++ for this

// suppose 3x6 array mat, K=1
//
// abcdefg
// hijklmn
// opqrstu
//
// then after calculating ret[0][0] = (a+b) + (h+i) + (o+p)
// we have
// ret[0][1] = ret[0][0] + (c+j)
// ret[0][2] = ret[0][1] + (d+k) - (a+h)
// ret[1][0] = ret[0][0] + (o+p)
// ret[1][1] = ret[0][1] + ret[1][0] - ret[0][0] + q
// and as such, we could calculate the return array in a *diagonal order*

class Solution {
  public:
    vector<vector<int>> matrixBlockSum(vector<vector<int>>& mat, int K) {

    }
};
