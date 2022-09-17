struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            // trivial corner case
            return 0;
        }

        let mut ret = if x == i32::MAX { x - 1 } else { x };
        // s.t. we won't have integer overflow later
        let mut quotient = 1;
        /*
         * 10? 10/1 = 10, (10+1)/2 = 5
         *     10/5 = 2, (5+2)/2 = 3
         *     10/3 = 3
         * Assuming integer operations here.
         */
        while ret > quotient {
            ret = (ret + quotient) / 2;
            quotient = x / ret;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(24), 4);
        assert_eq!(Solution::my_sqrt(25), 5);
        assert_eq!(Solution::my_sqrt(26), 5);
        assert_eq!(Solution::my_sqrt(35), 5);
        assert_eq!(Solution::my_sqrt(36), 6);
        assert_eq!(Solution::my_sqrt(i32::MAX), 46340);
    }
}
