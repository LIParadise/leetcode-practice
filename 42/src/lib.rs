/// Given n non-negative integers representing an elevation map where the width
/// of each bar is 1, compute how much water it can trap after raining.
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // Idea: remember walls
        //
        // A wall is s.t. where neighboring indices strictly decrease.
        //
        // Now we have the interesting part: having a wall,
        // where's the counterpart?
        //
        // Naive idea is that search for its partner till end, then again.
        // This has O(n^2) worse case; consider something like the following
        // [0, 100, 0, 99, 0, 98, 0, 97, 0, ..., 2, 0, 1, 0]
        //
        // Instead, we maintain a list of possible walls as follows:
        // First, record our wall.
        // For each number, if it's eligible for a wall, compare it with the
        // smallest member in our list.
        // While the new comer is no smaller, pop the smallest in the list,
        // since the new comer would submerge the smaller one.
        // If there's someone larger, stop, and append the new comer.
        // If new comer is no smaller than our last i.e. largest,
        // then the new comer IS the new wall,
        // we record this pair for later rain calculation.
        //
        // If till end we haven't depleted our list of possible walls,
        // this list records exactly where the walls are, and we're able to
        // calculate the total water.

        let mut ret = 0;

        let mut walls = Vec::new();
        walls.push(0);
        height.iter().enumerate().skip(1).for_each(|(i, &h)| {
            let mut l_backup = walls.last().map(|tup| tup.clone());
            while let Some(&l) = walls.last() {
                l_backup = walls.last().map(|tup| tup.clone());
                if h >= height[l] {
                    walls.pop();
                } else {
                    break;
                }
            }
            if walls.is_empty() {
                ret += Self::evaluate_walls(&height, l_backup.unwrap(), i);
            }
            walls.push(i);
        });
        walls.iter().zip(walls.iter().skip(1)).for_each(|(&l, &r)| {
            ret += Self::evaluate_walls(&height, l, r);
        });

        ret
    }
    fn evaluate_walls(h_arr: &[i32], l: usize, r: usize) -> i32 {
        let water_level = std::cmp::min(h_arr[l], h_arr[r]);
        h_arr[l + 1..r].iter().map(|i| water_level - i).sum()
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
