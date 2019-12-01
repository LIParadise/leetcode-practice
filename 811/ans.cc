#include <unordered_map>
#include <utility>
#include <cassert>
using namespace std;
class Solution {
  public:
    vector<string> subdomainVisits(vector<string>& cpdomains) {

      _myMap.clear();
      unordered_map<string, int>::iterator it = _myMap.begin();

      for( auto& itt : cpdomains ){
        for(temp_pos = 0; itt[temp_pos]!=' '&& temp_pos<itt.size() ; ++temp_pos);
        temp_token = itt.substr( 0, temp_pos );
        count = stoi( temp_token, nullptr, 10 );

        do{
          temp_pos++;
          temp_token = "";
          temp_token = itt.substr( temp_pos, string::npos);
          it = _myMap.find( temp_token );
          if( it != _myMap.end () ){
            it -> second += count;
          }else{
            _myMap.insert( pair< string, int > ( temp_token, count ) );
          }
        }
        while( _findDel( itt, temp_pos, '.' ) );
      }

      vector<string> ret;
      for( it = _myMap.begin(); it != _myMap.end(); it++ ){
        temp_token.clear();
        temp_token += to_string( it->second );
        temp_token += " ";
        temp_token += it->first;
        ret.push_back( move(temp_token) );
      }
      return ret;
    }



  private:
    unordered_map<string, int> _myMap;
    string temp_token;
    size_t temp_pos;
    int count;
    bool _findDel( string&, size_t&, char);
};
bool 
Solution::_findDel( string& str, size_t& pos, char del ){
  // assume format is correct.
  // that is, it's always "<integer> <domain name>"
  // where domain name is like "google.com.tw", delimiter is '.' and no delimiter trailing.
  // assume str[pos] is not del.
  if( pos == string::npos || pos >= str.size()){
    return false;
  }
  for(; str[pos] != del && pos < str.size(); ++pos );
  // we assume no ".." or "google.com.tw." in input;
  // i.e. assume they are overall good.
  return ( pos < str.size() );
}
