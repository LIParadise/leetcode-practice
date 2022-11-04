pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() <= 1 {
            0
        } else {
            Self::max_area_worker(&height)
        }
    }
    #[inline]
    fn make_container(left: usize, right: usize, height: &[i32]) -> i32 {
        min(height.get(right).unwrap(), height.get(left).unwrap()).clone() as i32
            * (right - left) as i32
    }
    fn max_area_worker(height: &[i32]) -> i32 {
        // assume length no less than 2
        let mut right = height.len() - 1;
        let mut left = 0;
        let mut ret = Self::make_container(left, right, height);
        while left < right {
            // need both wall to make container, hence less rather than leq
            let tmp = Self::make_container(left, right, height);
            if tmp > ret {
                ret = tmp;
            }
            match height.get(left).unwrap().cmp(height.get(right).unwrap()) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => {
                    left += 1;
                    right -= 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
