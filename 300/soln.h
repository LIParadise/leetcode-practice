#include <cstddef>
#include <vector>

/*
 * Given an integer array nums, return the length of the longest strictly
 * increasing subsequence.

 * A subsequence is a sequence that can be derived from an array by deleting
 * some or no elements without changing the order of the remaining elements. For
 * example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].
 */
class Solution {
    public:
        int lengthOfLIS(std::vector<int> &nums) {
            return static_cast<int>(longestMonotoneIncreasing(nums));
        }

    private:
        std::size_t longestMonotoneIncreasing(const std::vector<int> &);
};
