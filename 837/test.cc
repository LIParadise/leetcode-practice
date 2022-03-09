#include "soln.h"
#include <iostream>

int main() {
    Solution s;
    std::cout << s.new21Game(10, 1, 10) << std::endl;
    std::cout << s.new21Game(6, 1, 10) << std::endl;
    std::cout << s.new21Game(21, 17, 10) << std::endl;
    std::cout << s.new21Game(9632, 6575, 5602) << std::endl;

    return 0;
}
