//! Given an array of non-negative integers arr, you are initially positioned
//! at start index of the array. When you are at index i, you can jump to
//! i + arr[i] or i - arr[i], check if you can reach any index with value 0.
//!
//! Notice that you can not jump outside of the array at any time.
pub struct Solution;

use std::collections::VecDeque as Q;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        // BFS on the linear map.
        #[derive(Clone, Copy, Debug)]
        enum BfsState {
            Undiscovered,
            Working,
            Done,
        }
        let mut bfs_indices = Q::new();
        bfs_indices.push_front(start as usize);
        let mut states = vec![BfsState::Undiscovered; arr.len()];
        while !bfs_indices.is_empty() {
            let idx = bfs_indices.pop_back().unwrap();
            states[idx] = BfsState::Done;
            match arr[idx] {
                0 => return true,
                jump => {
                    if (idx + jump as usize) < arr.len() {
                        if let Some(BfsState::Undiscovered) = states.get(idx + jump as usize) {
                            states[idx + jump as usize] = BfsState::Working;
                            bfs_indices.push_front(idx + jump as usize);
                        }
                    }
                    if idx >= jump as usize {
                        if let Some(BfsState::Undiscovered) = states.get(idx - jump as usize) {
                            states[idx - jump as usize] = BfsState::Working;
                            bfs_indices.push_front(idx - jump as usize);
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
        assert!(Solution::can_reach(vec![0, 1], 1));
    }
}
