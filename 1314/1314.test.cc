// C version has some obsurd inputs to the function without description = =
// fuck leetcode
//
// so I chose C++ for this

// suppose 4x6 array mat, K=1
//
// abcdefg
// hijklmn
// opqrstu
// vwxyzAB
//
// observe the following:
// calculating ret[1][1]     calculating ret[1][2]
//      ~~~defg                   a~~~efg
//      ~~~klmn                   h~~~lmn
//      ~~~rstu                   o~~~stu
//      vwxyzAB                   vwxyzAB
// that is, we "push" the sub-column (d+k+r) into ret[1][1]
// and "pop" out the sub-column (a+h+o)
// hence, we need only remember these sub-column
// also, they are relatively easy to update:
// just remove the first row, and add an new row
// when considering ret[2][?]

#include <numeric>
#include <vector>
#include <iostream>
#include <iomanip>

using namespace std;

class Solution {
  public:
    vector<vector<int>> matrixBlockSum(vector<vector<int>>& mat, int K) {
      // prepare
      // m rows, n cols, e.g.
      // 000
      // 000
      // is a (m=2) times (n=3) zero matrix
      const int m = mat.size();
      const int n = mat[0].size();
      vector<vector<int>> ret;
      ret.resize( m );
      for( int i = 0; i < mat.size(); ++i ){
        ret[i].reserve ( n );
      }
      vector<int> subColumn;
      subColumn.reserve ( n );
      for( int i = 0; i < n; ++i ){
        subColumn.push_back(0);
        for( int j = 0; j < m && j <=K; ++j ){
          subColumn[i] += mat[j][i];
        }
      }

      for( int i = 0; i < m; ++i ){
        ret[i].push_back( accumulate(
              subColumn.begin(),
              subColumn.begin()+( (K<n)? (K+1) : n ),
              0 ) );
        int j = 1;
        for( ; j<=K && (j+K)<n ; ++j ){
          ret[i].push_back ( ret[i][j-1] + subColumn[j+K] );
        }
        for( ; (j+K)<n; ++j ){
          ret[i].push_back (
              ret[i][j-1] + subColumn[j+K] -
              ( (j-K-1)>=0 ? subColumn[j-K-1] : 0 )
              );
        }
        for( ; j<n; ++j ){
          ret[i].push_back (
              ret[i][j-1] -
              ( (j-K-1)>=0 ? subColumn[j-K-1] : 0 )
              );
        }

        // update "subColumn"
        j = 0;
        if( i < (m-1) ){
          for( ; j<n; ++j ){
            subColumn[j] = subColumn[j]
              + ( (i+K+1)<m?  mat[i+K+1][j] : 0)
              - ( (i-K  )>=0? mat[i-K  ][j] : 0);
          }
        }
      }

      return ret;
    } // vector<vector<int>> matrixBlockSum(vector<vector<int>>& mat, int K)
};

int main( int argc, char** argv ){

  int col = 0, row = 0;
  cout << "rows?" << endl;
  if ( cin >> row ){
    cout << "cols?" << endl;
    if (!( cin >> col )){
      cout << "RTFM" << endl;
      return 1;
    }
  }else {
    cout << "RTFM" << endl;
    return 1;
  }

  vector<vector<int>> mat;
  for( int i = 0; i < row; ++i ){
    vector<int> tmpVec;
    cout << "row " << i << endl;
    for( int j = 0; j < col; ++j ){
      int tmp = 0;
      if ( cin >> tmp ){
        tmpVec.push_back(tmp);
      }else{
        cout << "RTFM" << endl;
        return 1;
      }
    }
    mat.emplace_back(tmpVec);
  }

  cout << endl << "original:" << endl;
  ios_base::fmtflags cout_flags( cout.flags() );
  for( auto& r : mat ){
    for( auto& e : r ){
      cout << left << setw(7) << e;
    }
    cout << endl;
    cout.flags ( cout_flags );
  }

  cout << endl << endl << "operated:" << endl;

  // test start here
  Solution s;
  mat = s.matrixBlockSum ( mat, 1 );
  for( auto& r : mat ){
    for( auto& e : r ){
      cout << left << setw(7) << e;
    }
    cout << endl;
    cout.flags ( cout_flags );
  }

  return 0;
}
