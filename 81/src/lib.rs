pub struct Solution;

/// Search element in a "nearly" non-decreasing array
/// "Nearly" in that supp. one has a non-decreasing array, that array may go
/// under a cyclic group symmetry before passing to this function.
/// E.g. [0, 0, 1, 1, 2, 2] is non-decreasing, and that passed to this function
/// could be [1, 2, 2, 0, 0, 1]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        // Different from [LC Q.33](https://leetcode.com/problems/search-in-rotated-sorted-array/description/)
        // where the requirements are the same except that elements are
        // guaranteed to be distinct,
        // we can't use binary search to quickly determine where the cyclic
        // group operation is done.
        // Binary search in this case has worst case linear time.
        // So we just do linear scan to find where the rotation happened.

        let mut direct_find = false;
        let idx = nums
            .iter()
            .zip(nums.iter().skip(1))
            .position(|(curr, next)| {
                if curr == &target {
                    direct_find = true;
                }
                direct_find || curr > next
            })
            .map_or(nums.len() - 1, |idx| idx);
        if direct_find {
            return true;
        } else if idx == nums.len() - 1 {
            return nums.get(idx).map_or(false, |&val| val == target);
        } else {
            // No direct hit, but we know what offset is; binary search!
            struct Proxy<'a> {
                nums: &'a [i32],
                idx_of_largest: usize,
            }
            impl<'a> Proxy<'a> {
                /// As if no rotate happened, s.t. 0 returns smallest and
                /// (len-1) returns largest, assuming the array is indeed
                /// sorted non-decreasing with optional cyclic op.
                #[inline]
                fn virt_get(&self, idx: usize) -> Option<&i32> {
                    self.nums.get(self.virt_idx_in_real(idx))
                }
                /// Supp. arr A is non-decreasing, the struct holds a
                /// possibly rotated version of A
                /// Given index of A, return corresponding index in our ref.
                #[inline]
                fn virt_idx_in_real(&self, idx: usize) -> usize {
                    if idx + self.idx_of_largest + 1 < self.nums.len() {
                        idx + self.idx_of_largest + 1
                    } else {
                        idx - (self.nums.len() - self.idx_of_largest - 1)
                    }
                }
                fn position(&self, tgt: i32) -> Option<usize> {
                    let mut begin = 0;
                    let mut end = self.nums.len();
                    while begin < end {
                        let mid = (begin + end) / 2;
                        use std::cmp::Ordering::*;
                        match tgt.cmp(self.virt_get(mid).unwrap()) {
                            Less => end = mid,
                            Greater if begin == mid => return None,
                            Greater => begin = mid,
                            Equal => return Some(mid),
                        }
                    }
                    self.virt_get(begin).map_or(None, |&val| {
                        if val == tgt {
                            Some(self.virt_idx_in_real(begin))
                        } else {
                            None
                        }
                    })
                }
            }
            let p = Proxy {
                nums: &nums,
                idx_of_largest: idx,
            };
            p.position(target).is_some()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
