#include "soln.h"
#include <algorithm>
#include <cstddef>
#include <set>

/*
 * CLRS Introduction to Algorithms 3rd
 * Question 15.4-5
 * Give an O(n^2) time algorithm that gives the longest monotone
 * increasing subsequence of a subsequence of {n} numbers.
 */
std::size_t Solution::longestMonotoneIncreasing(const std::vector<int> &vec) {
    if (vec.size() <= 1) {
        return vec.size();
    }

    std::set<int> dp_helper;
    for (auto &a : vec) {
        dp_helper.insert(a);
    }
    std::vector<std::size_t> dp(vec.size(), 0);
    for (auto tail_itor = dp_helper.begin(); tail_itor != dp_helper.end();
         ++tail_itor) {
        const auto tail = *tail_itor;
        for (std::size_t col = 0; col < vec.size(); ++col) {
            while (col < vec.size() && vec.at(col) != tail) {
                ++col;
            }
            if (col < vec.size()) {
                const auto cur_dp_value = dp.at(col);
                for (; col < vec.size() && dp.at(col) == cur_dp_value; ++col) {
                    dp.at(col)++;
                }
            }
        }
    }
    return dp.back();
}
