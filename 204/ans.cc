#include <cmath>
#include <iostream>

using namespace std;

class Solution {
  public:
    int countPrimes(int n) {

      if( n<=2 )
        return 0;
      else if (n==3)
        return 1;
      else{

        int m = (sqrt(n) + 1 > n) ? n : sqrt(n) + 1, counter = 0;
        bool* ptr = new bool [ n+1 ] ();

        for( int i = 2; i <= m; ){
          for( int j = i+i; j <= n; j+=i ){
            ptr[j] = 1;
          }
          do{
            ++i;
          }while(i<=n && ptr[i] == 1);
        }
        ptr[0] = 1;
        ptr[1] = 1;
        ptr[n] = 1;

        for(int i = 0; i <= n; ++i ){
          if(ptr[i] == 0){
            ++counter;
          }
        }

        delete [] ptr;
        return counter;
      }

    }
};

int main(){
  Solution s;
  cout << "1: " << s.countPrimes(1) << endl;
  cout << "2: " << s.countPrimes(2) << endl;
  cout << "3: " << s.countPrimes(3) << endl;
  cout << "4: " << s.countPrimes(4) << endl;
  cout << "5: " << s.countPrimes(5) << endl;
  cout << "6: " << s.countPrimes(6) << endl;
  cout << "10: " << s.countPrimes(10) << endl;
  cout << "11: " << s.countPrimes(11) << endl;
  cout << "12: " << s.countPrimes(12) << endl;
  cout << "13: " << s.countPrimes(13) << endl;
  cout << "14: " << s.countPrimes(14) << endl;
  cout << "15: " << s.countPrimes(15) << endl;
  cout << "16: " << s.countPrimes(16) << endl;
  cout << "17: " << s.countPrimes(17) << endl;
  cout << "18: " << s.countPrimes(18) << endl;
  cout << "19: " << s.countPrimes(19) << endl;
  cout << "20: " << s.countPrimes(20) << endl;
  cout << "100000: " << s.countPrimes( 10*10000 ) << endl;
}
