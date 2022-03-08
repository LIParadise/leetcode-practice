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
#include <iomanip>
#include <iostream>
#include <ostream>
#include <utility>

/*
 * Suppose we want to solve for new21Game(n, k, M)
 * Conceptually, we maintain a {n \times k} table P
 * s.t. P(n, k) = new21Game(n, k, M)
 * Observe that columns in P(n, k) is CDF, s.t. when fix k = K,
 * P(i, K) <= P(j, K) iff i <= j.
 *
 * Not all entries in the {n \times k} table P are non-trivial.
 * Observe that given k = K, only { P(i, K) | i \in [K, K+M-1) } is
 * non-trivial,
 * alternatively, given n = N, only { P(N, i) | i \in (N-M+1, N] }
 * are non-trivial.
 * (Notice the difference between {[x, y]} and {(x, y)})
 * All the other entries are "trivial", in that they
 * equal to either 0.0 or 1.0.
 *
 * Upon further observation, one notices that updating entries can be done
 * in a sequential manner, which means one only need an array
 * of which length is "maxPts-1".
 * The { -1 } comes from the fact that to fully determine a PMF/CDF with known
 * set of finite supports (let's say {n} points), we only need to know the
 * probability mass of {n-1} of them.
 *
 * Back to the conceptual table P.
 * Suppose we know P[i, j]; what's the relation between it and P[i, j+1]?
 * (Let's assume P[i, j] and P[i, j+1] are both non-trivial)
 *
 * Let the card combinations that are to be counted in P[i, j] be C_{i, j}
 * P[i, j+1] be C_{i, j+1}, correspondingly.
 * C_{i, j} is the disjoint union of the following 2 sets:
 * One subset is those sum to j with the last card,
 * one subset is those sum to strictly larger than j with the last card.
 *
 * Consider C_{i, j+1}.
 * The latter subset falls entirely in C_{i, j+1},
 * while the former has fixed proportion to be classified as in C_{i, j+1},
 * with the addition of one more card to be drawn.
 * Why does the former behave as such?
 * By definition they sum to j, hence one more card is drawn, and as such
 * the combination would be in C_{i, j+1} iff the last card is in
 * {[1, i-j]}.
 *
 * One can check that C_{i, j+1} is comprised exactly of these
 * 2 kinds of combinations.
 *
 * Based on these we can do dynamic programming.
 */
double Solution::new21Game(int n, int k, int maxPts) {
    if (k == 0 || n >= k + maxPts - 1) {
        return 1.0;
    } else if (n < k) {
        return 0.0;
    }
    const double cp = 1.0 / ((double)maxPts); // uniform Card Probability
    dp_arr.resize(maxPts - 1);
    std::transform(
        dp_arr.begin(), dp_arr.end(), dp_arr.begin(),
        [&cp, this](auto &a) { return cp * ((&a - &dp_arr.at(0)) + 1); });
#ifdef LIPARADISE_DBG
    std::cout << "assume k is " << std::setw(2) << 1 << ": ";
    dump();
#endif // LIPARADISE_DBG
    for (int i = 2; i <= k; ++i) {
        const double prob_eq = *dp_arr.begin();
        // "prob_eq" denotes new21Game(i, i, maxPts)
        // Alternatively, using the terminology from above, P[i, i]
        // I.e. the probability that given we draw cards till no less than {i},
        // the probability that the sum is exactly i.
        // Hence the name, "probability of equality".
        for (std::size_t s = 0; s < dp_arr.size(); ++s) {
            const double subset_more =
                (-1.0) * prob_eq +
                ((s == dp_arr.size() - 1) ? 1.0 : dp_arr.at(s + 1));
            // "subset_more" denotes the probability of the second subset,
            // i.e. card combinations which sum to strictly larger than {i}.
            // Refer to the disjoint union analysis above.
            dp_arr.at(s) = (double)(s + 1) * cp * prob_eq + subset_more;
        }
#ifdef LIPARADISE_DBG
        std::cout << "assume k is " << std::setw(2) << i << ": ";
        dump();
#endif // LIPARADISE_DBG
    }
#ifdef LIPARADISE_DBG
    std::cout << "ans: " << dp_arr.at(n - k) << std::endl;
#endif // LIPARADISE_DBG
    return dp_arr.at(n - k);
}

#ifdef LIPARADISE_DBG
void Solution::dump() {
    std::ios_base::fmtflags orig_cout_flags(std::cout.flags());
    for (size_t s = 0; s < dp_arr.size(); ++s) {
        std::cout << std::right << std::setw(10) << std::setprecision(4)
                  << dp_arr.at(s);
    }
    std::cout << std::endl;
    std::cout.flags(orig_cout_flags);
}
#endif // LIPARADISE_DBG
