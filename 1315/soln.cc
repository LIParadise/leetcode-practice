#pragma once
#include "soln.h"
#include <vector>

namespace SolutionHelper {
int bfs_with_grand_parent_sum(TreeNode *n, std::vector<int> &gp) {
    if (!n) {
        return 0;
    }
    int sum = (gp.size() >= 2 && (*(gp.end() - 2)) % 2 == 0) ? n->val : 0;
    gp.emplace_back(n->val);
    sum += bfs_with_grand_parent_sum(n->right, gp);
    sum += bfs_with_grand_parent_sum(n->left, gp);
    gp.erase(gp.end() - 1);
    return sum;
}
} // namespace SolutionHelper

int Solution::sumEvenGrandparent(TreeNode *root) {
    // Data assumptions:
    // 1 <= # of nodes <= 10^4
    // 1 <= node.val <= 100

    using namespace SolutionHelper;
    std::vector<int> grandParents;
    grandParents.reserve(16); // 2^16 greater than 10^4;
    return bfs_with_grand_parent_sum(root, grandParents);
}
