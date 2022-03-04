/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left),
 * right(right) {}
 * };
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};

class Solution {
    public:
        int maxDepth(TreeNode *root) { return dfs_helper(root, 0); }

    private:
        const int dfs_helper(TreeNode *np, const int cur_depth) {
            if (np) {
                int depth = cur_depth + 1;
                const int r_depth = dfs_helper(np->right, cur_depth + 1);
                const int l_depth = dfs_helper(np->left, cur_depth + 1);
                depth = (r_depth > depth) ? r_depth : depth;
                depth = (l_depth > depth) ? l_depth : depth;
                return depth;
            } else {
                return cur_depth;
            }
        }
};
