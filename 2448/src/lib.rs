pub struct Solution;

/// You are given two 0-indexed arrays nums and cost consisting each of n positive integers.
///
/// You can do the following operation any number of times:
///     Increase or decrease any element of the array nums by 1.
/// The cost of doing one operation on the ith element is cost[i].
///
/// Return the minimum total cost such that all the elements of the array nums become equal.
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        if nums.is_empty() {
            return i64::default();
        }
        // Claim: if optimal cost, then result must be all input transformed
        // into some of the existing element, i.e. if [a_1, a_2, ..., a_n] then
        // result mut be [a_i, a_i, a_i, ..., a_i] where 1 <= i <= n.
        //
        // Proof:
        // It's trivial the result can't be either larger than max or smaller
        // than min.
        // Suppose WLOG [a_1, a_2, ..., a_n] sorted non-decreasingly, and that
        // [k, k, k, ..., k] reaches optimal cost, a_i <= k <= a_{i+1}
        // Then cost is \sum\limits_{d=1}^{i}{  (k-a_d)(c_d)  }
        //                                 +
        //              \sum\limits_{d=i+1}^{n}{  (a_d-k)(c_d)  }
        // Let C_A = \sum\limits_{d=1}^{i}{  c_d  }
        //     C_B = \sum\limits_{d=i+1}^{n}{  c_d  }
        // Notice if C_A != C_B, then [k, k, ..., k] must NOT be minimum, since
        // if WLOG C_A > C_B then we know [a_i, a_i, ..., a_i] is cheaper.
        // By premise it must be the case C_A == C_B, but this just implies
        // both [a_i; n] and [a_j; n] are also optimal.
        //
        // Further, notice if result array is [k; n], costs is correspondingly
        // a convex function of k.
        //
        // Proof:
        // Consider [k; n] as a_i <= k <= a_{i+1}
        // Each increase ([k^{\prime}; n] where k^{\prime} = k+1) takes cost
        // \sum\limits_{d=1}^{n}{  c_d  } - \sum\limits_{d=1}^{n}{  c_d  }
        // The former term is increasing, as the latter term is decreasing,
        // hence the function of cost of making [k; n] is composed of several
        // linear functions of k, and as k increases the slope increases.
        // One can check such a config is guaranteed to be convex.
        //
        // To sum up, for the modified array to be [k; n], corresponding cost is
        // a piece-wise linear function of k, with slope non-decreasing, a
        // convex function.
        // Further, if we plot for (k < min), slope must be negative,
        // since we're informed that costs are always positive.
        // Hence, to find optimal cost, just aim for the valley, i.e. when
        // slope becomes positive, or if slope maintains negative just use
        // the right-most.
        let mut nums_and_cost = Vec::with_capacity(nums.len());
        nums.into_iter()
            .zip(cost.into_iter())
            .for_each(|(n, c)| nums_and_cost.push((n as i64, c as i64)));
        nums_and_cost.sort_unstable_by(|(n1, _), (n2, _)| n1.cmp(n2));
        let slope_if_less_than_min: i64 = nums_and_cost.iter().map(|(_, c)| -c as i64).sum();
        let mut slope = slope_if_less_than_min;
        let mut idx = None;
        for i in 0..nums_and_cost.len() - 1 {
            slope = slope + nums_and_cost[i].1 * 2;
            if slope >= 0 {
                idx = Some(i);
                break;
            }
        }
        if let None = idx {
            idx = Some(nums_and_cost.len() - 1);
        }
        nums_and_cost.iter().fold(0, |accm, (n, c)| {
            ((n - nums_and_cost[idx.unwrap()].0) * c).abs() + accm
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]), 8);
        assert_eq!(
            Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
            0
        );
    }
}
