pub struct Solution;
pub use helper::*;

mod helper {
    use std::cmp::Ordering::*;
    pub use std::collections::BTreeMap as Map;
    use Interval as I;
    #[derive(Debug, Copy, Clone)]
    struct Interval<'a> {
        start: usize,
        end: usize,
        sum: i64,
        nums: &'a [i32],
    }
    impl<'a> Ord for I<'a> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(&other).unwrap()
        }
    }
    impl<'a> Eq for I<'a> {}
    impl<'a> PartialEq for I<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.sum.eq(&other.sum) && self.start.eq(&other.start)
        }
    }
    impl<'a> PartialOrd for I<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.sum.cmp(&other.sum) {
                Equal => Some(self.start.cmp(&other.start)),
                ordering => Some(ordering),
            }
        }
    }
    impl<'a> I<'a> {
        pub fn new((start, end): (usize, usize), nums: &'a [i32]) -> Option<Self> {
            if let Some(subarr) = nums.get(start..end) {
                Some(Self {
                    start,
                    end,
                    sum: subarr.iter().map(|&i| i as i64).sum(),
                    nums,
                })
            } else {
                None
            }
        }
    }
    impl<'a> I<'a> {
        pub fn get_sum(&self) -> i64 {
            self.sum
        }
        pub fn get_start(&self) -> usize {
            self.start
        }
        pub fn get_end(&self) -> usize {
            self.end
        }
        pub fn get_len(&self) -> usize {
            self.end - self.start
        }
        fn shrink_end(&mut self) -> Result<(), ()> {
            if self.start < self.end - 1 {
                self.end -= 1;
                self.sum -= self.nums[self.end] as i64;
                Ok(())
            } else {
                Err(())
            }
        }
        fn shrink_start(&mut self) -> Result<(), ()> {
            if self.start < self.end - 1 {
                self.sum += self.nums[self.start] as i64;
                self.start += 1;
                Ok(())
            } else {
                Err(())
            }
        }
        pub fn try_make_int_between(
            left: &Self,
            right: &Self,
            no_more_than: i64,
        ) -> Option<(Self, Self, Self, i64)> {
            assert!(std::ptr::eq(left.nums, right.nums));
            assert!(left.get_end() == right.get_start());
            let left_plus_right_sum_orig = left.get_sum() + right.get_sum();
            let mut left = left.clone();
            let mut right = right.clone();
            let (mut new_start, mut new_end) = (left.get_end(), left.get_end());
            while (left_plus_right_sum_orig - (left.get_sum() + right.get_sum())) <= no_more_than {
                match (left <= right, left.get_len() > 1, right.get_len() > 1) {
                    (_, false, false) => break,
                    (true, _, true) | (false, false, true) => {
                        right.shrink_start().unwrap();
                        new_end += 1;
                    }
                    (_, true, _) => {
                        left.shrink_end().unwrap();
                        new_start -= 1;
                    }
                }
            }
            Self::new((new_start, new_end), right.nums)
                .map(|i| (left, i, right))
                .map(|(a, b, c)| {
                    let mut max = a.get_sum();
                    if b.get_sum() > max {
                        max = b.get_sum();
                    }
                    if c.get_sum() > max {
                        max = c.get_sum();
                    }
                    (a, b, c, max)
                })
        }
    }

    pub struct PartitionManager<'a> {
        // When length is k, this array describes how to partition the array
        // into k non-empty subarrays s.t. infsup of sum among all methods to
        // partition into k non-empty subarrays is achieved
        intervals: Vec<I<'a>>,
        // Intervals, but sorted by their sum
        sorted_intervals: Map<I<'a>, usize>,
        // Want infsup of sum if partitioned in k non-empty subarrays?
        // It's at [k-1]
        // E.g. for there's only 1 interval, it's just whole array,
        // thus sum of array is sitting at [0].
        infsup_of_k_intervals: Vec<i64>,
    }
    impl<'a> PartitionManager<'a> {
        pub fn new(nums: &'a [i32]) -> Self {
            let mut intervals = Vec::new();
            let mut infsup_of_k_intervals = Vec::new();
            let mut sorted_intervals = Map::new();
            // When there's only 1 interval, it's just sum
            infsup_of_k_intervals.push(nums.iter().map(|&i| i as i64).sum());
            if nums.len() == 1 {
                let int = I::new((0, 1), nums).unwrap();
                intervals.push(int.clone());
                sorted_intervals.insert(int.clone(), 0);
                return Self {
                    intervals,
                    infsup_of_k_intervals,
                    sorted_intervals,
                };
            }

            // Consider when there's 2 non-empty interval
            // `[ [..=left_last], [left_last+1..] ]`
            let left_last = {
                let mut sum: i64 = 0;
                let left_last_cand = nums
                    .iter()
                    .position(|num| {
                        sum = sum + *num as i64;
                        sum >= infsup_of_k_intervals.first().copied().unwrap()
                    })
                    .unwrap();
                if sum * 2 == infsup_of_k_intervals.first().copied().unwrap() {
                    left_last_cand
                } else {
                    // [larger], [smaller]
                    // [..., z], [...]
                    //    l0      r0
                    // It's possible that move a element from l0 to r0
                    // yeilds smaller sup, i.e.
                    // [...], [z, ...]
                    //   l1,     r1
                    //
                    // Notice interval is denoted by [start..end],
                    // so here `l0` require an offset of 1.
                    let (l0, r0) = (
                        I::new((0, left_last_cand + 1), nums),
                        I::new((left_last_cand + 1, nums.len()), nums),
                    );
                    let (l1, r1) = (
                        I::new((0, left_last_cand), nums),
                        I::new((left_last_cand, nums.len()), nums),
                    );
                    let sup_0 = std::cmp::max(
                        l0.map(|i| i.get_sum()).unwrap_or(i64::MIN),
                        r0.map(|i| i.get_sum()).unwrap_or(i64::MIN),
                    );
                    let sup_1 = std::cmp::max(
                        l1.map(|i| i.get_sum()).unwrap_or(i64::MIN),
                        r1.map(|i| i.get_sum()).unwrap_or(i64::MIN),
                    );
                    if sup_0 > sup_1 {
                        // If there's an empty interval involved, the
                        // corresponding interval must have greater
                        // sum, forcing us to choose the other valid pair
                        left_last_cand - 1
                    } else if sup_0 == sup_1 {
                        unreachable!();
                    } else {
                        left_last_cand
                    }
                }
            };
            {
                let left = I::new((0, left_last + 1), nums).unwrap();
                let right = I::new((left_last + 1, nums.len()), nums).unwrap();
                intervals.push(left);
                intervals.push(right);
                sorted_intervals.insert(left, 0);
                sorted_intervals.insert(right, 1);
            }
            infsup_of_k_intervals.push(intervals.iter().max().unwrap().get_sum());

            Self {
                intervals,
                infsup_of_k_intervals,
                sorted_intervals,
            }
        }

        pub fn calc(&mut self, tgt_pars: usize) -> i32 {
            if let Some(i) = self.infsup_of_k_intervals.get(tgt_pars - 1) {
                *i as i32
            } else {
                for tgt in self.infsup_of_k_intervals.len()..=tgt_pars {
                    let (max, &idx) = self.sorted_intervals.iter().rev().next().unwrap();
                    let split_max_from_left = match self.intervals.get(idx.wrapping_sub(1)) {
                        Some(left) => Interval::try_make_int_between(
                            left,
                            max,
                            self.infsup_of_k_intervals[0] / tgt as i64,
                        ),
                        None => None,
                    };
                    let split_max_from_right = match self.intervals.get(idx + 1) {
                        Some(right) => Interval::try_make_int_between(
                            max,
                            right,
                            self.infsup_of_k_intervals[0] / tgt as i64,
                        ),
                        None => None,
                    };
                    match (split_max_from_left, split_max_from_right) {
                        (None, None) => break,
                        (Some(ints), None) => {
                            self.sorted_intervals
                                .remove(&self.intervals.remove(idx - 1));
                            self.sorted_intervals
                                .remove(&self.intervals.remove(idx - 1));
                            self.intervals.insert(idx - 1, ints.2.clone());
                            self.intervals.insert(idx - 1, ints.1.clone());
                            self.intervals.insert(idx - 1, ints.0.clone());
                            self.sorted_intervals.insert(ints.0, idx - 1);
                            self.sorted_intervals.insert(ints.1, idx);
                            self.sorted_intervals.insert(ints.2, idx + 1);
                            self.infsup_of_k_intervals.push(ints.3);
                        }
                        (None, Some(ints)) => {
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.intervals.insert(idx, ints.2.clone());
                            self.intervals.insert(idx, ints.1.clone());
                            self.intervals.insert(idx, ints.0.clone());
                            self.sorted_intervals.insert(ints.0, idx);
                            self.sorted_intervals.insert(ints.1, idx + 1);
                            self.sorted_intervals.insert(ints.2, idx + 2);
                            self.infsup_of_k_intervals.push(ints.3);
                        }
                        (Some(ints_l), Some(ints_r)) if ints_l.3 <= ints_r.3 => {
                            self.sorted_intervals
                                .remove(&self.intervals.remove(idx - 1));
                            self.sorted_intervals
                                .remove(&self.intervals.remove(idx - 1));
                            self.intervals.insert(idx - 1, ints_l.2.clone());
                            self.intervals.insert(idx - 1, ints_l.1.clone());
                            self.intervals.insert(idx - 1, ints_l.0.clone());
                            self.sorted_intervals.insert(ints_l.0, idx - 1);
                            self.sorted_intervals.insert(ints_l.1, idx);
                            self.sorted_intervals.insert(ints_l.2, idx + 1);
                            self.infsup_of_k_intervals.push(ints_l.3);
                        }
                        (Some(ints_l), Some(ints_r)) if ints_l.3 > ints_r.3 => {
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.intervals.insert(idx, ints_r.2.clone());
                            self.intervals.insert(idx, ints_r.1.clone());
                            self.intervals.insert(idx, ints_r.0.clone());
                            self.sorted_intervals.insert(ints_r.0, idx);
                            self.sorted_intervals.insert(ints_r.1, idx + 1);
                            self.sorted_intervals.insert(ints_r.2, idx + 2);
                            self.infsup_of_k_intervals.push(ints_r.3);
                        }
                        _ => unreachable!(),
                    }
                }
                while self.infsup_of_k_intervals.len() < tgt_pars {
                    self.infsup_of_k_intervals
                        .push(self.infsup_of_k_intervals.last().copied().unwrap());
                }
                self.infsup_of_k_intervals.last().copied().unwrap() as i32
            }
        }
    }
}

/// Given an integer array nums and an integer k, split nums into k non-empty
/// subarrays such that the largest sum of any subarray is minimized.
/// Return the minimized largest sum of the split.
/// A subarray is a contiguous part of the array.
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if k >= nums.len() {
            nums.iter().max().map_or(i32::MIN, |&i| i)
        } else {
            let mut im = PartitionManager::new(&nums);
            im.calc(k)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        // assert_eq!(Solution::split_array(vec![1, 2, 3], 2), 3);
        // assert_eq!(Solution::split_array(vec![1, 2, 3], 3), 3);
        // assert_eq!(Solution::split_array(vec![1, 2, 4], 2), 4);
        // assert_eq!(Solution::split_array(vec![1, 2, 4], 3), 4);
        // assert_eq!(Solution::split_array(vec![1, 2, 4], 100), 4);
        // assert_eq!(Solution::split_array(vec![1, 2, 4], 1), 7);
        // assert_eq!(
        //     Solution::split_array(vec![99, 1, 100, 95, 100, 1, 99], 5),
        //     100
        // );
        assert_eq!(
            Solution::split_array(vec![99, 1, 100, 95, 100, 1, 99], 4),
            195
        );
    }
}
