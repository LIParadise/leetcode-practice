#include <vector>

using namespace std;

class Solution {
public:
    double average(vector<int>& salary) {
        // salary.size() >= 3
        double sum = salary.at(0), min = salary.at(0), max = salary.at(0);
        for(size_t s = 1; s < salary.size(); ++s ){
            sum += salary.at(s);
            min = (min > salary.at(s))? salary.at(s) : min;
            max = (max < salary.at(s))? salary.at(s) : max;
        }
        sum -= min;
        sum -= max;
        return sum / ((double)salary.size()-2);
    }
};
