use core::num;

pub struct Solution;

#[derive(Debug)]
enum PartitionKind {
    Max,
    Min,
}

impl Solution {
    /// division-separated list, find optimal parenthesis s.t. output maximized
    pub fn optimal_division(nums: Vec<f32>) -> String {
        if nums.is_empty() {
            return String::new();
        }
        if nums.len() == 1 {
            return nums[0].to_string();
        }

        let mut max_dp = Self::allocate_dp(&nums);
        let mut min_dp = Self::allocate_dp(&nums);
        nums.iter()
            .enumerate()
            .zip(nums.iter().skip(1))
            .zip(max_dp.last_mut().unwrap().iter_mut())
            .zip(min_dp.last_mut().unwrap().iter_mut())
            .for_each(|((((idx, i), j), max), min)| {
                *max = Some((*i / *j, idx + 1));
                *min = *max
            });
        for sub_prob_len in 3..=nums.len() {
            for start_idx in 0..nums.len() - sub_prob_len + 1 {
                // case a. the first guy is the last parenthesized => trivial
                // e.g. the a in ((a)/(b/(c/d)/e))
                // case b. the first guy is NOT the last parenthesized => search
                // e.g. the a in ((a/b)/((c/d)/e))

                // case a.
                let mut max = (
                    nums[start_idx] as f32
                        / min_dp[nums.len() - (sub_prob_len - 1)][start_idx + 1]
                            .unwrap()
                            .0,
                    start_idx,
                );

                // case b.
                (2..sub_prob_len).for_each(|numerator_len| {
                    let numerator = max_dp[nums.len() - numerator_len][start_idx].unwrap().0;
                    let denominator = if numerator_len == sub_prob_len - 1 {
                        nums[start_idx + sub_prob_len - 1] as f32
                    } else {
                        min_dp[nums.len() - (sub_prob_len - numerator_len)]
                            [start_idx + numerator_len]
                            .unwrap()
                            .0
                    };
                    if numerator / denominator > max.0 {
                        max = (numerator / denominator, start_idx + numerator_len - 1);
                    }
                });

                max_dp[nums.len() - sub_prob_len][start_idx] = Some(max);

                // dual for min
                let mut min = (
                    nums[start_idx] as f32
                        / max_dp[nums.len() - (sub_prob_len - 1)][start_idx + 1]
                            .unwrap()
                            .0,
                    start_idx,
                );
                (2..sub_prob_len).for_each(|numerator_len| {
                    let numerator = min_dp[nums.len() - numerator_len][start_idx].unwrap().0;
                    let denominator = if numerator_len == sub_prob_len - 1 {
                        nums[start_idx + sub_prob_len - 1] as f32
                    } else {
                        max_dp[nums.len() - (sub_prob_len - numerator_len)]
                            [start_idx + numerator_len]
                            .unwrap()
                            .0
                    };
                    if numerator / denominator < min.0 {
                        min = (numerator / denominator, start_idx + numerator_len - 1);
                    }
                });
                min_dp[nums.len() - sub_prob_len][start_idx] = Some(min);
            }
        }
        println!("{:?}", max_dp);
        println!("{:?}", min_dp);
        let ret = Self::make_proto(&nums, nums.len(), 0, &max_dp, &min_dp, &PartitionKind::Max);
        let sret = String::from_utf8(ret.clone()).unwrap();
        dbg!(&sret);
        String::from_utf8(Self::trim(&ret)).unwrap()
    }

    fn make_proto(
        nums: &[f32],
        len: usize,
        start: usize,
        max_dp: &Vec<Vec<Option<(f32, usize)>>>,
        min_dp: &Vec<Vec<Option<(f32, usize)>>>,
        kind: &PartitionKind,
    ) -> Vec<u8> {
        let mut ret = Vec::with_capacity(len * 2);
        ret.push('(' as u8);
        match len {
            0 => panic!("Should not have to do len 0."),
            1 => ret.extend(nums[start].to_string().as_bytes()),
            2 => {
                ret.extend(nums[start].to_string().as_bytes());
                ret.push('/' as u8);
                ret.extend(nums[start + 1].to_string().as_bytes());
            }
            _ => {
                let (_, end) = match kind {
                    PartitionKind::Max => max_dp,
                    PartitionKind::Min => min_dp,
                }[nums.len() - len][start]
                    .unwrap();
                let singlet_numerator = end == start + len - 1;
                if singlet_numerator {
                    // case a.
                    ret.extend(nums[start].to_string().as_bytes());
                } else {
                    // case b.
                    ret.extend(Self::make_proto(
                        nums,
                        end - start + 1,
                        start,
                        max_dp,
                        min_dp,
                        kind,
                    ));
                }
                ret.push('/' as u8);
                ret.extend(Self::make_proto(
                    nums,
                    len - (end - start + 1),
                    end + 1,
                    max_dp,
                    min_dp,
                    match kind {
                        PartitionKind::Max => &PartitionKind::Min,
                        PartitionKind::Min => &PartitionKind::Max,
                    },
                ));
            }
        }
        ret.push(')' as u8);
        ret
    }

    fn trim(s: &[u8]) -> Vec<u8> {
        // input is overly parenthesized; remove them.
        // 1. the eqn. as a whole should not have parenthesis
        // 2. possible single number but parenthesized
        // 3. unnecessary parenthesis due to genuine order of evaluation, i.e. left to right

        let mut ret = s.to_owned();
        let mut stack = Vec::with_capacity(s.len());

        s.iter().enumerate().for_each(|(i, u)| match *u as char {
            '(' => {
                stack.push(i);
            }
            ')' => {
                stack.pop();
            }
            '/' => {}
            '0'..='9' | '.' => {}
            _ => {
                panic!("Unexpected u8")
            }
        });

        // 1.
        *ret.first_mut().unwrap() = 0;
        *ret.last_mut().unwrap() = 0;

        ret.into_iter()
            .filter_map(|u| (u != 0).then_some(u as u8))
            .collect()
    }

    // Triangular array, supp. input len is n,
    // len distribution of return is 1, 2, 3, ..., (n-1)
    // resp. storing parenthesis info for sub-problem of len n, n-1, n-2, ..., 2
    fn allocate_dp(nums: &Vec<f32>) -> Vec<Vec<Option<(f32, usize)>>> {
        let mut ret = Vec::with_capacity(nums.len());
        ret.extend((1..nums.len()).map(|i| {
            let mut tmp = Vec::with_capacity(i);
            tmp.extend((0..i).map(|_| None));
            tmp
        }));
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::optimal_division(vec![1000.0, 0.1, 0.2, 2.0]),
            "1000/(100/10/2)".to_owned()
        );
        /*
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)".to_owned()
        );
        assert_eq!(
            Solution::optimal_division(vec![2, 3, 4]),
            "2/(3/4)".to_owned()
        );
        */
    }

    fn test_build_dp() {
        let nums = vec![1.0];
        assert_eq!(
            Solution::allocate_dp(&nums),
            Vec::<Vec::<Option<(f32, usize)>>>::new()
        );
        let nums = vec![1.0, 2.0];
        assert_eq!(Solution::allocate_dp(&nums), vec![vec![None]]);
    }
}
