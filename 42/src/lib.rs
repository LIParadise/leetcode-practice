/// Given n non-negative integers representing an elevation map where the width
/// of each bar is 1, compute how much water it can trap after raining.
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // Idea:
        // [Reference](https://leetcode.com/problems/trapping-rain-water/solutions/3401992/100-detailed-explaination-with-pictures-in-c-java-python-two-pointers)
        //
        // Algorithm:
        //
        // Starting from both end, two pointers going inwards,
        // each step record how much water it'll collect,
        // while maintaining an auxiliary wall, i.e. the max height
        // they've encountered.
        //
        // While the two pointers going inwards:
        // If both pointers were now below the two auxiliary walls,
        // then we know an index that MUST be submerged,
        // i.e. the guy at the side of which auxiliary wall is shorter,
        // since it's between two walls.
        // Actually we know EXACTLY how deep:
        // it holds water till equal to its neighboring (shorter) auxiliary wall,
        // since that's maximum wall height recorded from that side.
        // Otherwise, either of the pointer is higher than previous wall,
        // update corresponding auxiliary wall.
        // This index would be dry after rain since there's no outer wall.

        let mut ret = 0;
        let mut l_ptr = height.iter().enumerate().peekable();
        let mut r_ptr = height.iter().rev().enumerate().peekable();
        let mut l_wall = 0;
        let mut r_wall = 0;
        while let (Some((l_idx, &l_h)), Some((r_idx, &r_h))) = (l_ptr.peek(), r_ptr.peek()) {
            if l_idx + r_idx >= height.len() {
                break;
            } else {
                if l_h > l_wall {
                    l_wall = l_h;
                    l_ptr.next();
                } else if r_h > r_wall {
                    r_wall = r_h;
                    r_ptr.next();
                } else if l_wall <= r_wall {
                    ret += l_wall - l_h;
                    l_ptr.next();
                } else {
                    ret += r_wall - r_h;
                    r_ptr.next();
                }
            }
        }
        r_ptr.next();

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        // before rain, terrain
        // [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
        // after rain, terrain with rain
        // [0, 1, 1, 2, 2, 2, 2, 3, 2, 2, 2, 1]
        // rain depth
        // [0, 0, 1, 0, 1, 2, 1, 0, 0, 1, 0, 0]
        // Sum is 6
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![1, 1]), 0);
        assert_eq!(Solution::trap(vec![1]), 0);
        assert_eq!(Solution::trap(vec![0]), 0);
        assert_eq!(
            Solution::trap(vec![
                1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, 0, 2, 0, 1
            ]),
            //  1  1  2  2  3  3  4  4  5  4  4  3  3  2  2  2  2  2  2  1  1
            //  0  1  0  2  0  3  0  4  0  4  0  3  0  2  0  2  1  2  0  1  0
            25
        );
    }
}
