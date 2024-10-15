use std::num::{NonZeroI32, NonZeroU32};

pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // idea:
        // check if dividend is larger than (divisor shift by some amount),
        // if so, that means a 1 in bit representation of quotient
        // assuming 2's complement

        // leetcode assumes input non-zero; use type to represent the fact
        Self::divide_worker(dividend, NonZeroI32::new(divisor).unwrap())
    }
    fn divide_worker(numerator: i32, denominator: NonZeroI32) -> i32 {
        let positive = (numerator >= 0) == (denominator.get() > 0);
        let numerator = numerator.unsigned_abs();
        let denominator = NonZeroU32::new(denominator.get().unsigned_abs()).unwrap();
        let max_shift_amount = {
            const BITS: usize = size_of::<i32>() * 8;
            BITS - 1
                - (0..BITS)
                    .filter(|&shift| (1 << shift) & denominator.get() == 1 << shift)
                    .last()
                    .unwrap()
        };
        let quotient = (0..=max_shift_amount)
            .rev()
            .scan(numerator, |numerator, shift| {
                if *numerator >= denominator.get() << shift {
                    *numerator -= denominator.get() << shift;
                    Some(1 << shift)
                } else {
                    Some(0)
                }
            })
            .sum::<u32>();

        // special cases
        if positive {
            if quotient > i32::MAX as u32 {
                i32::MAX
            } else {
                quotient as i32
            }
        } else {
            if quotient >= i32::MIN.unsigned_abs() {
                i32::MIN
            } else {
                -(quotient as i32)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
        assert_eq!(Solution::divide(i32::MAX, 1), i32::MAX);
        assert_eq!(Solution::divide(i32::MAX, -1), -i32::MAX);
        assert_eq!(Solution::divide(i32::MAX, 2), i32::MAX / 2);
        assert_eq!(Solution::divide(i32::MAX, -2), i32::MAX / -2);
        assert_eq!(Solution::divide(i32::MIN, 2), i32::MIN / 2);
        assert_eq!(Solution::divide(i32::MIN, -2), i32::MIN / -2);
        assert_eq!(Solution::divide(i32::MAX, 3), i32::MAX / 3);
        assert_eq!(Solution::divide(i32::MAX, -3), i32::MAX / -3);
        assert_eq!(Solution::divide(i32::MIN, 3), i32::MIN / 3);
        assert_eq!(Solution::divide(i32::MIN, -3), i32::MIN / -3);
    }
}
