pub struct Solution;

/// Given an integer n, return the number of structurally unique BST's (binary
/// search trees) which has exactly n nodes of unique values from 1 to n.
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // Consider BST with n nodes [1..=n]
        //
        // 1 => 1
        //
        // 2 => 1 (1 as root)
        //      +
        //      1 (2 as root)
        //      = 2
        //
        // 3 => 2 (3 as root, left subtree is size 2, which gives 2)
        //      +
        //      1*1 (2 as root, both left and right subtree are of size 1,
        //      +    which gives 1)
        //      2 (1 as root, right subtree is size 2, which gives 2)
        //      = 5

        // Both empty and size 1 are deemed as possessing 1 config
        let mut ans = Vec::from([1_usize, 1, 2, 5]);
        if n as usize <= 3 {
            ans[n as usize] as i32
        } else {
            while ans.len() <= n as usize {
                ans.push(ans.iter().zip(ans.iter().rev()).map(|(i, j)| i * j).sum())
            }
            *ans.last().unwrap() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(1), 1);
    }
}
