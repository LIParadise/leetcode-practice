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
                self.sum -= self.nums[self.start] as i64;
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
            let mut min = std::cmp::min(left.get_sum(), right.get_sum());
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
                        sum >= infsup_of_k_intervals.first().copied().unwrap() / 2
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
                    if sup_0 >= sup_1 {
                        // If there's an empty interval involved, the
                        // corresponding interval must have greater
                        // sum, forcing us to choose the other valid pair
                        left_last_cand - 1
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
                for tgt in self.infsup_of_k_intervals.len()..tgt_pars {
                    let (max, &idx) = self.sorted_intervals.iter().rev().next().unwrap();
                    let split_max_from_left = match self.intervals.get(idx.wrapping_sub(1)) {
                        Some(left) => Interval::try_make_int_between(
                            left,
                            max,
                            self.infsup_of_k_intervals[0] / (tgt + 1) as i64,
                        ),
                        None => None,
                    };
                    let split_max_from_right = match self.intervals.get(idx + 1) {
                        Some(right) => Interval::try_make_int_between(
                            max,
                            right,
                            self.infsup_of_k_intervals[0] / (tgt + 1) as i64,
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
                            self.sorted_intervals
                                .values_mut()
                                .filter(|i| **i > idx)
                                /* ??? */
                                .for_each(|i| *i += 1);
                            self.sorted_intervals.insert(ints.0, idx - 1);
                            self.sorted_intervals.insert(ints.1, idx);
                            self.sorted_intervals.insert(ints.2, idx + 1);
                            self.infsup_of_k_intervals.push(
                                self.sorted_intervals
                                    .iter()
                                    .rev()
                                    .next()
                                    .unwrap()
                                    .0
                                    .get_sum(),
                            );
                        }
                        (None, Some(ints)) => {
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.intervals.insert(idx, ints.2.clone());
                            self.intervals.insert(idx, ints.1.clone());
                            self.intervals.insert(idx, ints.0.clone());
                            self.sorted_intervals
                                .values_mut()
                                .filter(|i| **i > idx)
                                /* ??? */
                                .for_each(|i| *i += 1);
                            self.sorted_intervals.insert(ints.0, idx);
                            self.sorted_intervals.insert(ints.1, idx + 1);
                            self.sorted_intervals.insert(ints.2, idx + 2);
                            self.infsup_of_k_intervals.push(
                                self.sorted_intervals
                                    .iter()
                                    .rev()
                                    .next()
                                    .unwrap()
                                    .0
                                    .get_sum(),
                            );
                        }
                        (Some(ints_l), Some(ints_r)) if ints_l.3 <= ints_r.3 => {
                            self.sorted_intervals
                                .remove(&self.intervals.remove(idx - 1));
                            self.sorted_intervals
                                .remove(&self.intervals.remove(idx - 1));
                            self.intervals.insert(idx - 1, ints_l.2.clone());
                            self.intervals.insert(idx - 1, ints_l.1.clone());
                            self.intervals.insert(idx - 1, ints_l.0.clone());
                            self.sorted_intervals
                                .values_mut()
                                .filter(|i| **i > idx)
                                /* ??? */
                                .for_each(|i| *i += 1);
                            self.sorted_intervals.insert(ints_l.0, idx - 1);
                            self.sorted_intervals.insert(ints_l.1, idx);
                            self.sorted_intervals.insert(ints_l.2, idx + 1);
                            self.infsup_of_k_intervals.push(
                                self.sorted_intervals
                                    .iter()
                                    .rev()
                                    .next()
                                    .unwrap()
                                    .0
                                    .get_sum(),
                            );
                        }
                        (Some(ints_l), Some(ints_r)) if ints_l.3 > ints_r.3 => {
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.sorted_intervals.remove(&self.intervals.remove(idx));
                            self.intervals.insert(idx, ints_r.2.clone());
                            self.intervals.insert(idx, ints_r.1.clone());
                            self.intervals.insert(idx, ints_r.0.clone());
                            self.sorted_intervals
                                .values_mut()
                                .filter(|i| **i > idx)
                                /* ??? */
                                .for_each(|i| *i += 1);
                            self.sorted_intervals.insert(ints_r.0, idx);
                            self.sorted_intervals.insert(ints_r.1, idx + 1);
                            self.sorted_intervals.insert(ints_r.2, idx + 2);
                            self.infsup_of_k_intervals.push(
                                self.sorted_intervals
                                    .iter()
                                    .rev()
                                    .next()
                                    .unwrap()
                                    .0
                                    .get_sum(),
                            );
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
        assert_eq!(Solution::split_array(vec![1, 2, 3], 2), 3);
        assert_eq!(Solution::split_array(vec![1, 2, 3], 3), 3);
        assert_eq!(Solution::split_array(vec![1, 2, 4], 2), 4);
        assert_eq!(Solution::split_array(vec![1, 2, 4], 3), 4);
        assert_eq!(Solution::split_array(vec![1, 2, 4], 100), 4);
        assert_eq!(Solution::split_array(vec![1, 2, 4], 1), 7);
        assert_eq!(Solution::split_array(vec![2, 4, 3, 4, 4, 6], 3), 9);
        assert_eq!(Solution::split_array(vec![5, 2, 2, 1, 1, 3, 2], 3), 6);
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(
            Solution::split_array(vec![10, 5, 13, 4, 8, 4, 5, 11, 14, 9, 16, 10, 20, 8], 8),
            25
        );
        assert_eq!(
            Solution::split_array(vec![99, 1, 100, 95, 100, 1, 99], 5),
            100
        );
        assert_eq!(
            Solution::split_array(vec![99, 1, 100, 95, 100, 1, 99], 4),
            195
        );
        assert_eq!(
            Solution::split_array(
                vec![
                    5334, 6299, 4199, 9663, 8945, 3566, 9509, 3124, 6026, 6250, 7475, 5420, 9201,
                    9501, 38, 5897, 4411, 6638, 9845, 161, 9563, 8854, 3731, 5564, 5331, 4294,
                    3275, 1972, 1521, 2377, 3701, 6462, 6778, 187, 9778, 758, 550, 7510, 6225,
                    8691, 3666, 4622, 9722, 8011, 7247, 575, 5431, 4777, 4032, 8682, 5888, 8047,
                    3562, 9462, 6501, 7855, 505, 4675, 6973, 493, 1374, 3227, 1244, 7364, 2298,
                    3244, 8627, 5102, 6375, 8653, 1820, 3857, 7195, 7830, 4461, 7821, 5037, 2918,
                    4279, 2791, 1500, 9858, 6915, 5156, 970, 1471, 5296, 1688, 578, 7266, 4182,
                    1430, 4985, 5730, 7941, 3880, 607, 8776, 1348, 2974, 1094, 6733, 5177, 4975,
                    5421, 8190, 8255, 9112, 8651, 2797, 335, 8677, 3754, 893, 1818, 8479, 5875,
                    1695, 8295, 7993, 7037, 8546, 7906, 4102, 7279, 1407, 2462, 4425, 2148, 2925,
                    3903, 5447, 5893, 3534, 3663, 8307, 8679, 8474, 1202, 3474, 2961, 1149, 7451,
                    4279, 7875, 5692, 6186, 8109, 7763, 7798, 2250, 2969, 7974, 9781, 7741, 4914,
                    5446, 1861, 8914, 2544, 5683, 8952, 6745, 4870, 1848, 7887, 6448, 7873, 128,
                    3281, 794, 1965, 7036, 8094, 1211, 9450, 6981, 4244, 2418, 8610, 8681, 2402,
                    2904, 7712, 3252, 5029, 3004, 5526, 6965, 8866, 2764, 600, 631, 9075, 2631,
                    3411, 2737, 2328, 652, 494, 6556, 9391, 4517, 8934, 8892, 4561, 9331, 1386,
                    4636, 9627, 5435, 9272, 110, 413, 9706, 5470, 5008, 1706, 7045, 9648, 7505,
                    6968, 7509, 3120, 7869, 6776, 6434, 7994, 5441, 288, 492, 1617, 3274, 7019,
                    5575, 6664, 6056, 7069, 1996, 9581, 3103, 9266, 2554, 7471, 4251, 4320, 4749,
                    649, 2617, 3018, 4332, 415, 2243, 1924, 69, 5902, 3602, 2925, 6542, 345, 4657,
                    9034, 8977, 6799, 8397, 1187, 3678, 4921, 6518, 851, 6941, 6920, 259, 4503,
                    2637, 7438, 3893, 5042, 8552, 6661, 5043, 9555, 9095, 4123, 142, 1446, 8047,
                    6234, 1199, 8848, 5656, 1910, 3430, 2843, 8043, 9156, 7838, 2332, 9634, 2410,
                    2958, 3431, 4270, 1420, 4227, 7712, 6648, 1607, 1575, 3741, 1493, 7770, 3018,
                    5398, 6215, 8601, 6244, 7551, 2587, 2254, 3607, 1147, 5184, 9173, 8680, 8610,
                    1597, 1763, 7914, 3441, 7006, 1318, 7044, 7267, 8206, 9684, 4814, 9748, 4497,
                    2239
                ],
                9
            ),
            194890
        );
    }
}
