pub struct Solution;

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let nums = {
                let mut v = Vec::with_capacity(nums.len() + 2);
                v.push(1);
                v.append(&mut nums);
                v.push(1);
                v
            };

            // To reduce memory footprint, access needs some index hack:
            //
            // Given subproblem with left sentinel index and right sentinel index,
            // the access to the DP memoization is [left][right - left - 2]
            //
            // E.g. consider `[3, 1, 4, 1, 5]`,
            // after the expansion with virtual sentinels we get `[1, 3, 4, 1, 5, 1]`,
            // and the problem is asking left sentinel idx 0 and right sentinel idx 5,
            // and it could be found at `[0][5 - 0 - 2]` i.e. `[0][3]`
            // similarly for subproblem `[3, 4, 1, 5]` it's `[1][4 - 1 - 2]` i.e. `[1][1]`
            let mut ddp: Vec<Vec<Option<i32>>> =
                Vec::from_iter((0..nums.len() - 2).map(|left_sentinel| {
                    Vec::from_iter((left_sentinel + 2..nums.len()).map(|_right_sentinel| None))
                }));

            let ret = Self::max_coins_worker(&nums, (0, nums.len() - 1), &mut ddp);
            ret
        }
    }

    /// Given list of balloons and two ordered indices which cannot be popped,
    /// consider the balloons in between:
    /// what's the best strategy popping the in-between balloons?
    ///
    /// The algorithm starts by choosing one by one the *last* balloon to be popped, making the
    /// problem recursive and applicable for dynamic programming.
    fn max_coins_worker(
        nums: &[i32],
        (left_sentinel, right_sentinel): (usize, usize),
        ddp: &mut [Vec<Option<i32>>],
    ) -> i32 {
        // both `left_sentinel` and `right_sentinel` are NOT to be popped
        //
        // choose one index to be the LAST one popped;
        // such config is easy to calculate the points
        //
        // iterate over all possible indices, then find max.
        if left_sentinel + 1 >= right_sentinel {
            0
        } else {
            let subproblem_ans = match ddp.get_mut(left_sentinel).and_then(|v|
                    // memory footprint indexing hack;
                    // see the calling function for why this is the case
                    v.get_mut(right_sentinel - left_sentinel - 2))
            {
                Some(None) => {
                    // `last` larger, so minus 1 is OK
                    (left_sentinel..right_sentinel)
                        .skip(1)
                        .map(|mid_sentinel| {
                            let left =
                                Self::max_coins_worker(nums, (left_sentinel, mid_sentinel), ddp);
                            let right =
                                Self::max_coins_worker(nums, (mid_sentinel, right_sentinel), ddp);
                            left + right
                                + nums[mid_sentinel] * nums[left_sentinel] * nums[right_sentinel]
                        })
                        .max()
                        .unwrap()
                }
                Some(Some(dp_entry)) => *dp_entry,
                None => unreachable!(),
            };

            *ddp.get_mut(left_sentinel)
                .and_then(|v|
                    // memory footprint indexing hack;
                    // see the calling function for why this is the case
                    v.get_mut(right_sentinel - left_sentinel - 2))
                .unwrap() = Some(subproblem_ans);
            subproblem_ans
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_coins(vec![1, 2]), 4);
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);

        {
            // first 1 => 23;
            // first 3 => 25;
            // first 5 => 21;
            // max is 25
            assert_eq!(Solution::max_coins(vec![1, 3, 5]), 25);
        }

        {
            // pop 3 then 1 => 3 + 5 + 48 = 56,
            // pop 1 then 3 => 15 + 15 + 48 = 78,
            //
            // pop 3 then 5 => 3 + 40 + 16 = 59,
            // pop 5 then 3 => 40 + 3 + 16 = 59,
            //
            // pop 3 then 8 => 3 + 40 + 10 = 53,
            // pop 8 then 3 => 40 + 3 + 10 = 53,
            //
            // pop 1 then 5 => 15 + 120 + 32 = 167,
            // pop 5 then 1 => 40 + 24 + 32 = 96,
            //
            // pop 1 then 8 => 15 + 40 + 20 = 75,
            // pop 8 then 1 => 40 + 15 + 20 = 75,
            //
            // pop 5 then 8 => 40 + 8 + 6 = 54,
            // pop 8 then 5 => 40 + 5 + 6 = 51,
            assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
        }
    }
}
