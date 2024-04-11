pub struct Solution;

impl Solution {
    // Brute force
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let target = nums.iter().fold(0, |target, n| target | n);
        let mut ret = 0;
        Self::powerset_bitwise_or_count_worker(&target, &vec![], 0, &nums, &mut ret);
        ret.try_into().unwrap()
    }
    fn powerset_bitwise_or_count_worker(
        target: &i32,
        part: &[i32],
        index: usize,
        set: &[i32],
        powerset_size: &mut usize,
    ) {
        match set.get(index) {
            Some(&i) => {
                Self::powerset_bitwise_or_count_worker(target, part, index + 1, set, powerset_size);
                let mut part = part.to_owned();
                part.push(i);
                Self::powerset_bitwise_or_count_worker(
                    target,
                    &part,
                    index + 1,
                    set,
                    powerset_size,
                );
            }
            None => {
                if part.iter().fold(0, |or_result, i| i | or_result) == *target {
                    *powerset_size += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
