pub struct Solution;

/// Given an integer array nums and an integer k, split nums into k non-empty
/// subarrays such that the largest sum of any subarray is minimized.
/// Return the minimized largest sum of the split.
/// A subarray is a contiguous part of the array.
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        // Binary search for infsup
        // Guess a upper bound for the array, say `u`;
        // `u` is a upper bound iff there's certain partition scheme
        // s.t. sup of sum is no greater than `u`.
        //
        // Problem is, given `u`, is there a strategy to find a suitable
        // partition?
        // Isn't it possible that `u` is indeed an upper bound but it's our
        // lack of ability to find a partition proving its validity?
        //
        // Claim: Greedy Choice is such strategy.
        // Greedy: starting from first, pick as many element as possible as
        // long as their sum no larger than `u` to form subarray
        //
        // Observe:
        // 1. Greedy strategy gives unique partition scheme.
        // 2. If A is subarr of B, length of A >= k, u >= infsup of sum of B,
        //    then u >= infsup of sum of A, both partition into `k` subarrays
        //    I.e. infsup of sum of k non-empty subarr is "non-decreasing"
        //
        // Proof:
        // Induction, for any length `n`, induct on all corresponding valid `k`
        // (i.e. `k <= n`)
        //
        // For when ther's single subarr, this is obvious.
        // Supp. for all `k <= S` greedy works for all upper bounds thrown at
        // it, consider `k = S+1` and `u` being an upper bound.
        // By `u` upper bound, there exists partition `[p_1, p_2, ..., p_{S+1}]`
        // that gives inf(sup(sum(p_i))) <= u
        // By nature of greedy and observation 1, greedy must have chosen a
        // subarr that contains `p_1` as subarr, which means `u` is upper bound
        // for infsup of sum of `[p2, ..., p_{S+1}]` with `S` subarrs.
        // By observation 2, subsequent choices of greedy is effectively
        // solving an array with upper bound no larger than `u`, and by
        // induction hypothesis, since it's only `S` subarr to deal with,
        // subsequent choices of greedy must yield infsup of sum no greater
        // than `u`.
        // Combine with the first subarr, of which sum is also no greater than
        // `u`, we know the greedy choice indeed gives valid partition scheme.

        let k = k as usize;
        if k > nums.len() || nums.len() == 0 {
            panic!(
                "Can't generate {k} non-empty subarr since length is only {}",
                nums.len()
            );
        } else if k == nums.len() {
            return nums.into_iter().max().unwrap();
        } else {
            let max_element = nums.iter().max().clone().unwrap();
            // binary search, criteria being greedy choice
            let (mut start, mut end) =
                (*max_element, max_element * (nums.len() / k + 1) as i32 + 1);
            while start + 1 < end {
                let upperbound_cand = (start + end - 1) / 2;
                // start < start+1 <= end
                // mid = (start + end - 1)/2
                // start <= mid <= end-1 < end
                let mut sum = None;
                let interval_cnt_of_greedy = nums
                    .iter()
                    .filter(|&&n| match sum {
                        // Greedy strategy
                        // Count how many intervals are there by counting their
                        // start elements
                        Some(s) if s + n <= upperbound_cand => {
                            sum = Some(s + n);
                            false
                        }
                        _ => {
                            sum = Some(n);
                            true
                        }
                    })
                    .take(k + 1)
                    .count();
                if interval_cnt_of_greedy == k + 1 {
                    // candidate upper bound too small;
                    // try larger?
                    start = upperbound_cand + 1;
                } else {
                    // valid upper bound;
                    // want infsup, hence search in lower part
                    end = upperbound_cand + 1;
                }
            }
            start
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
