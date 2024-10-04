pub struct Solution;
pub(crate) use std::collections::HashMap;

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut dp = HashMap::<(usize, usize), i32>::new();
            let nums = {
                let mut v = Vec::with_capacity(nums.len() + 2);
                v.push(1);
                v.append(&mut nums);
                v.push(1);
                v
            };
            let ret = Self::max_coins_worker(&nums, (0, nums.len() - 1), &mut dp);
            ret
        }
    }
    fn max_coins_worker(
        nums: &[i32],
        sentinels @ (left_sentinel, right_sentinel): (usize, usize),
        dp: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        // both `first` and `last` are NOT to be popped
        //
        // choose one index to be the LAST one popped;
        // such config is easy to calculate the points
        //
        // iterate over all possible indices, then find max.

        if left_sentinel + 1 >= right_sentinel {
            0
        } else {
            if dp.get(&sentinels).is_none() {
                // `last` larger, so minus 1 is OK
                let ret = (left_sentinel..right_sentinel)
                    .skip(1)
                    .map(|mid_sentinel| {
                        let left = Self::max_coins_worker(nums, (left_sentinel, mid_sentinel), dp);
                        let right =
                            Self::max_coins_worker(nums, (mid_sentinel, right_sentinel), dp);
                        left + right
                            + nums[mid_sentinel] * nums[left_sentinel] * nums[right_sentinel]
                    })
                    .max()
                    .unwrap();
                dp.insert(sentinels, ret);
            }
            dp.get(&sentinels).cloned().unwrap()
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
