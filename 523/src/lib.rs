pub struct Solution;

mod moduloz {

    #[derive(Debug, Clone, Copy, Eq, Ord)]
    pub struct ModuloZ {
        u: usize,
        n: usize,
    }
    impl PartialOrd for ModuloZ {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.n != other.n {
                None
            } else {
                Some(self.u.cmp(&other.u))
            }
        }
    }
    impl PartialEq for ModuloZ {
        fn eq(&self, other: &Self) -> bool {
            self.n == other.n && self.u == other.u
        }
    }
    impl std::ops::Sub for ModuloZ {
        type Output = Self;
        fn sub(mut self, rhs: Self) -> Self::Output {
            if self.n != rhs.n {
                panic!("Cannot calc. (Z_{} - Z_{})", self.n, rhs.n);
            } else {
                self.tick(self.n - rhs.u);
                self
            }
        }
    }
    impl std::ops::Add for ModuloZ {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self::Output {
            if self.n != rhs.n {
                panic!("Cannot calc. (Z_{} - Z_{})", self.n, rhs.n);
            } else {
                self.tick(rhs.u);
                self
            }
        }
    }
    impl ModuloZ {
        const MACHINE_MAX: usize = usize::MAX;
        pub fn new(n: usize, u: usize) -> Self {
            let mut ret = Self { n, u: 0 };
            ret.tick(u);
            ret
        }
        pub fn new_i32(n: usize, i: i32) -> Self {
            let mut zero = Self::new(n, 0);
            zero.tick_i32(i);
            zero
        }
        pub fn tick_i32(&mut self, i: i32) {
            match i.cmp(&0) {
                std::cmp::Ordering::Greater => self.tick(i as usize),
                std::cmp::Ordering::Less => {
                    let to_subtract = Self::new(self.n, -i as usize);
                    *self = *self - to_subtract;
                }
                _ => {}
            }
        }
        pub fn tick(&mut self, increment: usize) {
            let increment = if increment >= self.n {
                increment % self.n
            } else {
                increment
            };
            let old_u = self.u;
            self.u = self.u.wrapping_add(increment);
            if self.u < old_u {
                self.tick(Self::MACHINE_MAX - self.n);
                self.tick(1);
            } else if self.u >= self.n {
                self.u %= self.n
            }
        }
    }
}

impl Solution {
    /// Given an integer array nums and an integer k, return true if nums has a
    /// good subarray or false otherwise.
    ///
    /// A good subarray is a subarray where:
    /// its length is at least two,
    /// it's contiguous,
    /// the sum of the elements of the subarray is a multiple of k.
    ///
    /// 0 is considered to be multiple of all integer.
    // Idea:
    // Create auxiliary array with same length, on which records
    // the remainder of all PREVIOUS entries, self INCLUDED.
    // Additionally, for all possible remainder (k, hence [0..k]), record
    // the minimum index in the auxiliary array which got that remainder.
    // As we fill up the auxiliary array, we can check if there's match:
    // remainders match with sufficient distance (leetcode asks 2), done.
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use crate::moduloz::ModuloZ as MZ;
        use std::collections::BTreeMap as BTM;
        let zero = MZ::new(k as usize, 0);
        let k = k as usize;
        let mut remainder_of_sum_subarr = zero;
        let mut remainder_lookup_table = BTM::new();
        nums.iter()
            .enumerate()
            .try_for_each(|(idx, &num)| {
                remainder_of_sum_subarr = remainder_of_sum_subarr + MZ::new_i32(k, num);
                if remainder_of_sum_subarr == zero && idx >= 1 {
                    Err(())
                } else {
                    remainder_lookup_table.entry(remainder_of_sum_subarr).or_insert(idx);
                    match remainder_lookup_table.get(&remainder_of_sum_subarr) {
                        Some(i) if idx >= i + 2 => Err(()),
                        _ => Ok(()),
                    }
                }
            })
            .is_err()
    }
}

#[cfg(test)]
mod tests {
    use crate::moduloz::ModuloZ;
    use crate::Solution;
    #[test]
    fn test_modulo() {
        let mut z7 = ModuloZ::new(7, 4);
        z7.tick(4);
        assert_eq!(z7, ModuloZ::new(7, 1));
        z7.tick(16);
        assert_eq!(z7, ModuloZ::new(7, 3));
        z7.tick(253);
        assert_eq!(z7, ModuloZ::new(7, 4));
        let z7_3 = ModuloZ::new(7, 3);
        let z7_neg_4 = ModuloZ::new_i32(7, -4);
        let z7_6 = ModuloZ::new(7, 6);
        assert_eq!(z7_3 + z7_neg_4, z7_6);
        let z11_4 = ModuloZ::new(11, 4);
        let z11_5 = ModuloZ::new(11, 5);
        assert_eq!(z11_4 - z11_5, ModuloZ::new(11, 10));
    }
    #[test]
    fn test_soln() {
        assert!(Solution::check_subarray_sum(Vec::from([23, 2, 4, 6, 7]), 6));
        assert!(Solution::check_subarray_sum(Vec::from([23, 2, 6, 4, 7]), 6));
        assert!(Solution::check_subarray_sum(Vec::from([1, 2, 3]), 5));
        assert!(!Solution::check_subarray_sum(
            Vec::from([23, 2, 6, 4, 7]),
            13
        ));
        assert!(!Solution::check_subarray_sum(Vec::from([23, 6, 9]), 6));
    }
}
