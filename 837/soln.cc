/*
 * Alice plays the following game, loosely based on the card game "21".
 *
 * Alice starts with 0 points and draws numbers while she has less than k
 * points. During each draw, she gains an integer number of points randomly from
 * the range [1, maxPts], where maxPts is an integer. Each draw is independent
 * and the outcomes have equal probabilities.
 *
 * Alice stops drawing numbers when she gets k or more points.
 *
 * Return the probability that Alice has n or fewer points.
 *
 * Answers within 10-5 of the actual answer are considered accepted.
 */

#include "soln.h"
#include <algorithm>
#include <cstddef>
#include <numeric>
#include <utility>

/* Follow method provided by lee215@leetcode
 * https://leetcode.com/problems/new-21-game/discuss/132334/One-Pass-DP-O(N)
 *
 * First, note that when given K and M, "new21Game(i, K, M)" is only
 * interesting when {i \in [K, K+M-1)}, otherwise the result is just
 * 1.0 or 0.0
 *
 * We'll do dynamic programming where we maintain the probability that
 * some integer "i" is to be sum of some card combination,
 * where {i \in [1, K+M-1]},
 * with the constraint we stop draw cards when {i \geq K}.
 * (Denote them as P(i))
 *
 * The cards come from uniform i.i.d distribution.
 * Observe that once all P(i) are collected, a simple Bayes' Theorem
 * gives the answer to "new21Game(n, K, M)", that is,
 * let "bayes_whole_prob" be
 * {
 *      {\sum\limits_{i=K}^{K+M-1}{P(i)}
 * }
 * and "bayes_subset_prob" be
 * {
 *      {\sum\limits_{i=K}^{N}{P(i)}
 * }
 * Then we just return "bayes_subset_prob/bayes_whole_prob".
 *
 * Notice that for {i < K}, P(i) can be obtained by
 * {
 *      {\sum\limits_{j=i-M}^{i-1}{P_j}} \times
 *      {\frac{1}{M}}
 * }
 * The reason is again Bayes' theorem; want to sum to i?
 * Then consider (prob that sum to i-1) * (prob that draw 1)
 *               (prob that sum to i-2) * (prob that draw 2)
 *               (prob that sum to i-3) * (prob that draw 3)
 * so on and so forth
 */
double Solution::new21Game(int n, int k, int maxPts) {
    // Input range assumptions:
    // 0 <= k <= n <= 10^4
    // 1 <= maxPts <= 10^4

    if (n >= k + maxPts - 1 || k == 0) {
        return 1.0;
    } else if (n < k) {
        return 0.0;
    }

    dp_arr = std::vector<double>(maxPts + 1, 0.0);
    const double cardProb = 1.0 / ((double)maxPts);
    dp_arr.front() = 1.0;
    double sum_of_dp_arr = dp_arr.front();

    // Bayes' theorem
    for (std::size_t i = 1; i < k + maxPts; ++i) {
        const double prob_of_i = sum_of_dp_arr * cardProb;
        if (i >= maxPts) {
            sum_of_dp_arr -= dp_arr.at((i - maxPts) % dp_arr.size());
        }
        if (i < k) {
            sum_of_dp_arr += prob_of_i;
        }
        dp_arr.at(i % dp_arr.size()) = prob_of_i;
    }

    double bayes_whole_prob = 0.0;
    for (std::size_t s = k; s <= k + maxPts - 1; ++s) {
        bayes_whole_prob += dp_arr.at(s % dp_arr.size());
    }
    double bayes_subset_prob = 0.0;
    for (std::size_t s = k; s <= n; ++s) {
        bayes_subset_prob += dp_arr.at(s % dp_arr.size());
    }

    return bayes_subset_prob / bayes_whole_prob;
}
