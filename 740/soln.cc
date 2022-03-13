#include "soln.h"
#include <algorithm>

/*
 * You are given an integer array nums. You want to maximize the number of
 * points you get by performing the following operation any number of times:
 *
 * Pick any nums[i] and delete it to earn nums[i] points.
 * Afterwards, you must delete every element equal to nums[i] - 1
 * and every element equal to nums[i] + 1.
 *
 * Return the maximum number of points you can earn by applying the above
 * operation some number of times.
 */
int Solution::deleteAndEarn(std::vector<int> &nums) {
    // If some number n is isolated, in that if there's neither (n+1)
    // nor (n-1) in "nums", n must be collected.
    // Otherwise, supp. we have consecutive numbers {[n, m]}
    // Then we inspect if the even numbers have greater sum or the odd,
    // and collect the higher one.
    // In fact, that the sum is from even numbers or odd numbers doesn't
    // matter. It's the alternating sum that matters.
    std::sort(nums.begin(), nums.end());
    int ret = 0;
    for (auto i0 = nums.begin(), i1 = nums.begin(); i1 != nums.end(); i0 = i1) {
        int partial_sums[2] = {0, 0};
        partial_sums[(*i0) % 2] += *i0;
        ++i1;
        while (i1 != nums.end()) {
            switch (*i1 - *i0) {
            case 1:
                i0 = i1;
            case 0:
                partial_sums[(*i0) % 2] += *i0;
                ++i1;
                continue;
            }
            break;
        }
        ret += (partial_sums[0] > partial_sums[1]) ? partial_sums[0]
                                                   : partial_sums[1];
    }
    return ret;
}
