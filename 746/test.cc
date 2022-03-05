#include "soln.h"
#include <iostream>

int main() {
    Solution s;
    std::vector<int> test0 = {10, 15, 20};
    std::vector<int> test1 = {1, 100, 1, 1, 1, 100, 1, 1, 100, 1};
    int result = s.minCostClimbingStairs(test0);
    std::cout << "[10,15,20]: " << result << std::endl;
    result = s.minCostClimbingStairs(test1);
    std::cout << "[1,100,1,1,1,100,1,1,100,1]: " << result << std::endl;

    return 0;
}
