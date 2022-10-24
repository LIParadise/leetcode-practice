pub struct Solution;

impl Solution {
    fn gcd(mut i: usize, mut j: usize) -> usize {
        if i == 0 || j == 0 {
            return 0;
        } else {
            // i       = j       * q_0     + r_0
            // j       = r_0     * q_1     + r_1
            // r_0     = r_1     * q_2     + r_2
            // ...
            // r_{i-1} = r_i     * q_{i+1} + r_{i}
            // r_i     = r_{i+1} * q_{i+2} + 0
            // Then r_{i+1} = GCD of a and b.
            let mut r;
            while {
                r = i % j;
                r != 0
            } {
                i = j;
                j = r;
            }
            j
        }
    }
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        let k = k as usize;

        // The problem asks only for contiguous subarr, even hints for
        // brute-force. Hence do brute-force here.
        // We check [0..1], [0..2], ..., [0..len]
        // then             [1..2], ..., [1..len]
        //                                    ...
        //                               [len-1..len]
        // For each row, notice GCD of subarr is non-increasing:
        // if GCD of set S is G, then GCD of set (S union {num}) is just
        // GCD of G and num.
        // Hence, for each row, we maintain a variable, s.t. column by column
        // we can do GCD calculation just once, i.e. just do GCD of the old
        // variable and the newly added element.
        for start in 0..nums.len() {
            // Maximum possible GCD of each row,
            // decreases as subarr longer and longer.
            let mut gcd_of_subarr = unsafe {
                // unsafe reason: get_unchecked
                // It's okay since here we iterate on known valid boundary
                *nums.get_unchecked(start) as usize
            };
            for end in start + 1..nums.len() + 1 {
                unsafe {
                    // unsafe reason: get_unchecked
                    // It's okay since here we iterate on known valid boundary
                    gcd_of_subarr = Self::gcd(
                        *nums.get_unchecked(start..end).last().unwrap() as usize,
                        gcd_of_subarr,
                    );
                    if gcd_of_subarr == k {
                        cnt += 1;
                    } else if gcd_of_subarr < k {
                        // The larger the set the smaller the GCD
                        // If it's below k then just prune unnecessary calc
                        break;
                    }
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_gcd() {
        assert_eq!(Solution::gcd(10, 3), 1);
        assert_eq!(Solution::gcd(3, 11), 1);
        assert_eq!(Solution::gcd(100, 84), 4);
    }
    #[test]
    fn test_soln() {
        assert_eq!(Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
        assert_eq!(Solution::subarray_gcd(vec![4], 7), 0);
    }
}
