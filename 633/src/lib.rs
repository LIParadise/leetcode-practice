pub struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        match c.cmp(&0) {
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Greater => Self::judge_square_sum__(c as u32),
        }
    }
    fn judge_square_sum__(u: u32) -> bool {
        // two pointers: supp. u = a^2 + b^2,
        // WLOG supp. a <= b <= sqrt(u),
        // then we may do two-pointers in the range `0..=sqrt(u)`
        let mut left = 0;
        let mut right = Self::diy_sqrt(u);
        loop {
            match (left * left + right * right).cmp(&u) {
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Equal => break true,
            }
            if left > right {
                break false;
            }
        }
    }
    pub(crate) fn diy_sqrt(u: u32) -> u32 {
        static MAX_U32_SQRT: u32 = (1_u32 << (std::mem::size_of::<u32>() * 8 / 2)) - 1;
        match u {
            0 => 0,
            sqrt_is_one if (1..=3).contains(&sqrt_is_one) => 1,
            u => {
                let mut sqrt_candidate = 2;
                loop {
                    if u >= sqrt_candidate * sqrt_candidate {
                        if u - sqrt_candidate * sqrt_candidate == (2 * sqrt_candidate + 1) {
                            break sqrt_candidate + 1;
                        } else if u - sqrt_candidate * sqrt_candidate < 2 * sqrt_candidate + 1 {
                            break sqrt_candidate;
                        }
                    }
                    let quotient = u / sqrt_candidate;
                    sqrt_candidate = std::cmp::min((sqrt_candidate + quotient) / 2, MAX_U32_SQRT);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_soln() {
        assert!(Solution::judge_square_sum(0));
        assert!(Solution::judge_square_sum(1));
        assert!(Solution::judge_square_sum(2));
        assert!(!Solution::judge_square_sum(3));
        assert!(Solution::judge_square_sum(4));
        assert!(Solution::judge_square_sum(5));
        assert!(!Solution::judge_square_sum(6));
    }

    #[test]
    fn test_diy_sqrt() {
        (1..100).chain(u32::MAX - 100..=u32::MAX).for_each(|u| {
            let diy_sqrt = Solution::diy_sqrt(u);
            assert!(u >= diy_sqrt * diy_sqrt);
            assert!(u - diy_sqrt * diy_sqrt < 2 * diy_sqrt + 1);
            assert!((diy_sqrt - 1) * (diy_sqrt - 1) < u);
        });
    }
}
