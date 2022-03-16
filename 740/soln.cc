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
    /* Dynamic programming
     *
     * Suppose we have {k} distinct numbers, {n_1, n_2, ..., n_k},
     * we can store a {k \times k} table (let the table be T)
     * s.t. T[i][j] marks the answer to the {n_i, n_{i+1}, ..., n_{j}}
     * For example, suppose "nums" is the following
     * [1, 1, 3, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6]
     * Then T[2][3] stores the answer to the following:
     *       [3, 4, 5, 5, 5, 5, 5, 5]
     * Notice first that in "T", it stores the order statistics, not the
     * actual number in "nums"
     * Notice further that {i \leq j} always holds.
     * (We only need to store half of a square matrix)
     *
     * Suppose we know T[i][j] and T[j+1][k], what's T[i][k]?
     * If the j-th smallest and the (j+1)-th smallest differ by
     * larger or equal to 2, then T[i][k] = T[i][j] + T[j+1][k]
     * (The ordering is done ignoring all equal terms)
     *
     * If the j-th and the (j+1)-th differ by 1,
     * then we need to consider 2 candidates,
     * one being T[i][j-1] + T[j+1][k], one being T[i][j] + T[j+2][k].
     * Those T[m][n] with {m > n} are deemed as 0.
     */

    std::sort(nums.begin(), nums.end());
    std::size_t distinct_terms = 0; // k; see comment above
    for (auto i = nums.begin(), j = nums.begin(); j != nums.end(); i = j) {
        ++distinct_terms;
        while (j < nums.end() && *j == *i) {
            ++j;
        }
    }
    TriangularMatrix T(distinct_terms);
    std::vector<int> values(distinct_terms, 0);
    std::size_t idx = 0;
    for (auto itor_0 = nums.begin(), itor_1 = nums.begin();
         itor_1 != nums.end(); itor_0 = itor_1, ++idx) {
        int sum = 0;
        while (itor_1 != nums.end() && *itor_1 == *itor_0) {
            sum += *itor_1;
            ++itor_1;
        }
        T.set(idx, idx, sum);
        values.at(idx) = *itor_0;
    }

    for (idx = 1; idx < distinct_terms; ++idx) {
        for (std::size_t offset = 0; offset < distinct_terms - idx; ++offset) {
            // Calculate the subproblem T[row][col]
            // of which values range from the (row)-th smallest to the (col)-th
            const std::size_t row = offset;
            const std::size_t col = offset + idx;
            int sum = 0;

            // For T[row][col], there are many ways to cut subproblem.
            // For each subproblem, there are 4 cases to consider.
            // 1. They are "disjoint", i.e. in sorted array, they differ by
            // larger than scalar 1
            // 2.(a) choose (m), discard (m+1)
            // 2.(b) choose (m+1), discard (m)
            // 2.(c) discard both (m) and (m+1)
            for (std::size_t s = 0; s < idx; ++s) {
                const std::pair<std::size_t, std::size_t> lhs =
                    std::make_pair(row, row + s);
                decltype(lhs) rhs = std::make_pair(row + s + 1, col);
                int tmpSum = 0;
                const int case1 =
                    T.at(lhs.first, lhs.second) + T.at(rhs.first, rhs.second);
                const int case2a = T.at(lhs.first, lhs.second) +
                                   T.at(rhs.first + 1, rhs.second);
                const int case2b = T.at(lhs.first, lhs.second - 1) +
                                   T.at(rhs.first, rhs.second);
                const int case2c = T.at(lhs.first, lhs.second - 1) +
                                   T.at(rhs.first + 1, rhs.second);
                tmpSum = (values.at(lhs.second) + 1 < values.at(rhs.first) &&
                          case1 > tmpSum)
                             ? case1
                             : tmpSum;
                tmpSum = (case2a > tmpSum) ? case2a : tmpSum;
                tmpSum = (case2b > tmpSum) ? case2b : tmpSum;
                tmpSum = (case2c > tmpSum) ? case2c : tmpSum;
                if (tmpSum > sum) {
                    sum = tmpSum;
                }
            }
            T.set(row, col, sum);
        }
    }

    return T.at(0, distinct_terms - 1);
}
