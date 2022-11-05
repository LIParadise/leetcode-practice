pub struct Solution;

impl Solution {
    /// Array of *distinct* numbers sorted in ascending order
    /// Possibly "rotated" in that maybe it's (originally [1..=n])
    /// [n-m, n-m+1, ..., n, 1, 2, 3, ..., n-m-1]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering::*;
        // corner cases
        if nums.is_empty() {
            return -1;
        }
        if nums.len() == 1 {
            return if nums.last().unwrap().eq(&target) {
                0
            } else {
                -1
            };
        }

        // 2-pass search
        // 1st, find where the largest went, create proxy index
        // s.t. we could index as if no rotation happened
        // 2nd, b-search using proxy index
        // 3rd, translate proxy index into real

        // find where rotate happen
        let mut start = 0;
        let mut end = nums.len();
        while start < end {
            // invariant:
            // there's a decrease either in subarr or right after subarr
            if start == end - 1 {
                break;
            } else {
                match nums.get(start).cmp(&nums.get(end - 1)) {
                    Less => start = end - 1,
                    Greater => {
                        let mid = (start + end) / 2;
                        match nums.get(mid).cmp(&nums.get(end - 1)) {
                            Greater => start = mid,
                            Less => end = mid,
                            Equal => {
                                // given promise of distinct elements, this means
                                // they are same index
                                assert_eq!(start + 1, mid);
                                assert_eq!(mid + 1, end);
                                // only 2 elements
                                match nums.get(start).cmp(&nums.get(mid)) {
                                    Greater => end = mid,
                                    Less => start = mid,
                                    Equal => {
                                        panic!("The promise of distinct elements is broken")
                                    }
                                }
                            }
                        }
                    }
                    _ => panic!("The promise of distinct elements is broken"),
                }
            }
        }

        // `start` is where rotate happen
        // Illustration: supp. originally [0..n]
        // with rotation s.t.
        // nums:  [m, m+1, ..., n-1, 0, 1, 2, ..., m-1]
        // index:  0   1      n-m-1 n-m            n-1
        //                   (n-m-1) is exactly `start`
        //                              also `orig_last_idx`
        // Similarly, if no rotation happened at all, `start` would be at (len-1)
        struct Proxy<'a> {
            nums: &'a [i32],
            orig_last_idx: usize,
        }
        impl<'a> Proxy<'a> {
            fn get_non_rotated(&self, idx: usize) -> Option<&i32> {
                match self.nums.get(idx + self.orig_last_idx + 1) {
                    ret @ Some(_) => ret,
                    None => self
                        .nums
                        .get(idx - (self.nums.len() - self.orig_last_idx - 1)),
                }
            }
            fn search(&self, tgt: i32) -> Option<usize> {
                let mut begin = 0;
                let mut end = self.nums.len();
                let mut ret = None;
                while begin < end {
                    let mid = (begin + end) / 2;
                    match tgt.cmp(self.get_non_rotated(mid).unwrap()) {
                        Less if begin == mid => break,
                        Less => end = mid,
                        Equal => {
                            ret = Some(mid);
                            break;
                        }
                        Greater => begin = mid + 1,
                    }
                }
                ret.map(|r| {
                    if r < (self.nums.len() - self.orig_last_idx - 1) {
                        r + self.orig_last_idx + 1
                    } else {
                        r - (self.nums.len() - self.orig_last_idx - 1)
                    }
                })
            }
        }
        let pg = Proxy {
            nums: &nums,
            orig_last_idx: start,
        };
        pg.search(target).map_or(-1, |u| u as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![3, 1], 3), 0);
        assert_eq!(Solution::search(vec![5, 7, 8, 0, 3, 4], 7), 1);
    }
}
