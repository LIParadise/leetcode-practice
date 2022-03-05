#include "soln.h"

int Solution::minCostClimbingStairs(std::vector<int> &cost) {
    /*
     * We'll use dynamic programming.
     * Suppose cost = [ 1 , 100 , 1 , 1 , 1 , 100 , 1 , 1 , 100 , 1]
     *                ^
     * Conceptually, we start at caret, and decides to land on the next
     * immediate or the next immediate of the next immediate. To see the
     * optimal substructure for dynamic programming to work, consider
     * suppose we decide to take 1 step for our first choice of stair; the
     * question now becomes cost = [ 1 , 100 , 1 , 1 , 1 , 100 , 1 , 1 , 100
     * , 1]
     *                    ^
     * And if this is part of some optimal way of choosing stairs,
     * it must follows an optimal way to walk the remaining stairs.
     */
    /*
     *
     *  2 <= cost.length <= 1000
     *  0 <= cost[i] <= 999
     */

    if (cost.size() == 2) {
        return cost.at(0) <= cost.at(1) ? cost.at(0) : cost.at(1);
    }
    /*
     * We'll take an bottom-up approach, starting from back of array.
     * Base cases are [ x , y , z ] and [ x , y , z ]
     *                        ^             ^
     * And then consider [ x , y , z ]
     *                   ^
     */
    // auxiliary costs for dynamic programming
    std::vector<int> partial_cost(cost.size(), 0);
    // auxiliary strategies for dynamic programming
    std::vector<bool> strategy(cost.size(), false);
    // in "strategy", false means immediate next, true means one more

    // Base cases:
    partial_cost.back() = 0;
    strategy.back() = true;
    if (cost.at(cost.size() - 1) <= cost.at(cost.size() - 2)) {
        partial_cost.at(partial_cost.size() - 2) = cost.at(cost.size() - 1);
        strategy.at(strategy.size() - 2) = true;
    } else {
        partial_cost.at(partial_cost.size() - 2) = cost.at(cost.size() - 2);
        strategy.at(strategy.size() - 2) = false;
    }
    // bottom-up approach
    for (std::size_t s = cost.size() - 2; s > 0;) {
        --s;
        int walk_cost = cost.at(s);     // choose next immediate
        int jump_cost = cost.at(s + 1); // choose 1 pass next immediate
        walk_cost += partial_cost.at(s + 1);
        jump_cost += partial_cost.at(s + 2);
        if (walk_cost < jump_cost) {
            strategy.at(s) = false;
            partial_cost.at(s) = walk_cost;
        } else {
            strategy.at(s) = true;
            partial_cost.at(s) = jump_cost;
        }
    }
    return *partial_cost.begin();
}
