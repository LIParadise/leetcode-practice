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
     * consider atomic case, i.e. {k = 1} and all numbers are just {n_1}
     * then we just return the sum of `nums`.
     *
     * Suppose we've solved {k = N}, consider {k = N+1}
     * Then there are 3 cases.
     * 1. both {n_N} and {n_{N+1}} are taken
     * 2.(a) {n_N} is taken, while {n_{N+1}} is not.
     * This means the result is exactly the same as when {k=N}.
     * 2.(b) {n_{N+1}} is taken, while {n_N} is discarded.
     * Which means the result becomes (that of {k={N-1}}) plus
     * (sum of {n_{K+1}}).
     *
     * Case 1 happens only when {n_N + 1 < n_{N+1}}
     * Case 2 happens when {n_N + 1 == n_{N+1}}
     * There can't be the case when both {n_N} and {n_{N+1}} are
     * not taken, since they are different numbers.
     */

    std::sort(nums.begin(), nums.end());
    std::size_t distinct_terms = 0; // k; see comment above
    for (auto i = nums.begin(), j = nums.begin(); j != nums.end(); i = j) {
        ++distinct_terms;
        while (j < nums.end() && *j == *i) {
            ++j;
        }
    }
    std::vector<int> sumOfIndividualTerms(distinct_terms, 0);
    std::vector<int> ansOfSubproblems(distinct_terms, 0);
    std::vector<int> individualTerms(distinct_terms, 0);
    for (auto n = nums.begin(), m = nums.begin(),
              s = sumOfIndividualTerms.begin(), i = individualTerms.begin();
         m < nums.end(); n = m) {
        while (m < nums.end() && *m == *n) {
            ++m;
            *s += *n;
        }
        *i = *n;
        ++s;
        ++i;
    }
    ansOfSubproblems.front() = sumOfIndividualTerms.front();
    for (std::size_t N = 0; (N + 1) < distinct_terms; ++N) {
        if (individualTerms.at(N) + 1 < individualTerms.at(N + 1)) {
            // case 1
            ansOfSubproblems.at(N + 1) =
                ansOfSubproblems.at(N) + sumOfIndividualTerms.at(N + 1);
        } else {
            // case 2
            const int case2a = ansOfSubproblems.at(N);
            const int case2b = ((N > 0) ? ansOfSubproblems.at(N - 1) : 0) +
                               sumOfIndividualTerms.at(N + 1);
            ansOfSubproblems.at(N + 1) = (case2a > case2b) ? case2a : case2b;
        }
    }

    return ansOfSubproblems.back();
}
