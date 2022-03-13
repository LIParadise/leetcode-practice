#include "soln.h"
#include <algorithm>
#include <iostream>

int main() {
    std::vector<int> test;
    int expected;
    Solution s;

    test = std::vector<int>(
        {10, 8, 4,  2, 1,  3,  4, 8,  2, 9, 10, 4, 8,  5, 9, 1,  5, 1,  6,  8,
         1,  1, 6,  7, 8,  9,  1, 7,  6, 8, 4,  5, 4,  1, 5, 9,  8, 6,  10, 6,
         4,  3, 8,  4, 10, 8,  8, 10, 6, 4, 4,  4, 9,  6, 9, 10, 7, 1,  5,  3,
         4,  4, 8,  1, 1,  2,  1, 4,  1, 1, 4,  9, 4,  7, 1, 5,  1, 10, 3,  5,
         10, 3, 10, 2, 1,  10, 4, 1,  1, 4, 1,  2, 10, 9, 7, 10, 1, 2,  7,  5});
    expected = 338;
    std::cout << "Expected: " << expected << ", my: " << s.deleteAndEarn(test)
              << std::endl;
    int even = 0, odd = 0;
    std::for_each(test.begin(), test.end(), [&even, &odd](const auto &a) {
        if (a % 2) {
            odd += a;
        } else {
            even += a;
        }
    });
    std::cout << "Odd: " << odd << ", Even: " << even << std::endl;

    return 0;
}
