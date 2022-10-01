pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

mod soln_helper {
    use std::ops::Add;

    #[derive(Eq, Debug)]
    pub struct TripletCandidate<'a, T: Eq + Clone + Default + Add<Output = T>> {
        pub idx_triplet: [usize; 3],
        difference: usize,
        nums: &'a Vec<T>,
    }

    mod soln_helper_impls {
        use super::TripletCandidate;
        use std::ops::Add;
        impl<'a, T: Eq + Clone + Default + Add<Output = T>> TripletCandidate<'a, T> {
            pub fn new(idx_triplet: [usize; 3], difference: usize, nums: &'a Vec<T>) -> Self {
                Self {
                    idx_triplet,
                    difference,
                    nums,
                }
            }
            pub fn get(&self) -> T {
                self.idx_triplet
                    .iter()
                    .fold(T::default(), |ret, index| ret + self.nums[*index].clone())
            }
        }
        impl<'a, T: Eq + Clone + Default + Add<Output = T>> PartialEq for TripletCandidate<'a, T> {
            fn eq(&self, other: &Self) -> bool {
                self.difference == other.difference
            }
        }
        impl<'a, T: Eq + Clone + Default + Add<Output = T>> PartialOrd for TripletCandidate<'a, T> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.difference.partial_cmp(&other.difference)
            }
        }
        impl<'a, T: Eq + Clone + Default + Add<Output = T>> Ord for TripletCandidate<'a, T> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.difference.cmp(&other.difference)
            }
        }
    }
}

impl Solution {
    /// Given an integer array nums of length n and an integer target, find
    /// three integers in nums such that the sum is closest to target.
    ///
    /// Return the sum of the three integers.
    ///
    /// You may assume that each input would have exactly one solution.
    // 3 <= nums.length <= 1000
    // -1000 <= nums[i] <= 1000
    // -10^4 <= target <= 10^4
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // O(n^2*ln(n)) with binary search.
        // First, sort the array
        // Then, consider the triplet [0th, 1st, ?]
        //
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^
        //
        // By sorted, binary search in [2nd..] for a number at nth s.t. the
        // triplet [0th, 1st, nth] >= 0 and all those mth with (m < n) leads
        // to triplet [0th, 1st, mth] with negative sum.
        // Record either sum of [0th, 1st, mth] or [0th, 1st, nth].
        //
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^              ...    (binary searching...)
        // Array: 01xxxxxxxxxxxxxxnxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^              ^      (hit)
        //
        // Repeat the process for [0th, 2nd, ?], [0th, 3rd, ?], ...
        // which is O(n^2) times.

        use soln_helper::TripletCandidate as TC;

        let mut nums = nums;
        nums.sort_unstable();
        match nums
            .iter()
            .enumerate()
            .fold(
                BinaryHeap::new(),
                |mut ret_heap: BinaryHeap<Reverse<TC<i32>>>, (idx, i)| {
                    nums.as_slice()[idx + 1..].iter().enumerate().for_each(
                        |(jdx_with_offset, j)| {
                            // i_idx, j_idx, k_idx => idx, jdx, kdx
                            let jdx = jdx_with_offset + idx + 1;
                            assert_eq!(nums[jdx], *j);
                            let kdx_candidate = nums.as_slice()[jdx + 1..]
                                // k_candidate => kandidate
                                .partition_point(|kandidate| i + j + kandidate <= target)
                                + jdx
                                + 1;
                            // kdx_candidate is partition_point
                            // hence the order of subtraction
                            if kdx_candidate == nums.len() {
                                if jdx != nums.len() - 1 {
                                    ret_heap.push(Reverse(TC::new(
                                        [idx, jdx, nums.len() - 1],
                                        (target - (i + j + nums.last().unwrap())) as usize,
                                        &nums,
                                    )));
                                }
                            } else {
                                let mut tc_candidate = TC::new(
                                    [idx, jdx, kdx_candidate],
                                    (i + j + nums[kdx_candidate] - target) as usize,
                                    &nums,
                                );
                                // It's possible partition_point is not the closest to target
                                // Check if valid neighbor index
                                // If valid and smaller, use the neighbor.
                                if jdx + 1 != kdx_candidate {
                                    tc_candidate = std::cmp::min(
                                        TC::new(
                                            [idx, jdx, kdx_candidate - 1],
                                            (target - (i + j + nums[kdx_candidate - 1])) as usize,
                                            &nums,
                                        ),
                                        tc_candidate,
                                    );
                                }
                                ret_heap.push(Reverse(tc_candidate));
                            }
                        },
                    );
                    ret_heap
                },
            )
            .pop()
        {
            Some(Reverse(ret)) => ret.get(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_soln() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
        assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0], 1));
    }
}
