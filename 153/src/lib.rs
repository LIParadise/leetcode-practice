pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let idx_of_min = Self::rotated_sorted_unique_find_min_index(&nums);
        idx_of_min
            .and_then(|i| nums.get(i).cloned())
            .unwrap_or(i32::MIN)
    }

    fn rotated_sorted_unique_find_min_index(rotated_sorted_unique: &[i32]) -> Option<usize> {
        (!rotated_sorted_unique.is_empty()).then(|| {
            let mut left = 0;
            let mut right = rotated_sorted_unique.len() - 1;
            // invariant:
            // the min is of index in the range `left..=right`
            //
            // note in particular it's more than loop invariant:
            // the invariant is hold even after the loop break!
            while left < right {
                let mid = Self::mid(left, right);
                if rotated_sorted_unique[mid] > rotated_sorted_unique[right] {
                    // `mid` must NOT be index of min,
                    // since it's larger than some other guy,
                    // so we may exclude it.
                    //
                    // `mid` is calculated s.t. it's always strictly smaller than `right`
                    // thx to semantics of unsigned integer division is RTZ
                    left = mid + 1;
                } else {
                    right = mid
                }
            }
            left
        })
    }

    fn mid(a: usize, b: usize) -> usize {
        a.checked_add(b)
            .map(|sum| sum / 2)
            .unwrap_or_else(|| a / 2 + b / 2 + (a % 2 + b % 2) / 2)
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        // unimplemented!()
    }

    #[test]
    fn test_utils() {
        let v = [];
        assert_eq!(Solution::rotated_sorted_unique_find_min_index(&v), None);

        let v = [1];
        assert_eq!(Solution::rotated_sorted_unique_find_min_index(&v), Some(0));

        let v = [1, 2];
        assert_eq!(Solution::rotated_sorted_unique_find_min_index(&v), Some(0));

        let v = [3, 1, 2];
        assert_eq!(Solution::rotated_sorted_unique_find_min_index(&v), Some(1));

        let v = Vec::from_iter((0..100_i32).map(|i| i.wrapping_add(i32::MAX - 42)));
        assert_eq!(Solution::rotated_sorted_unique_find_min_index(&v), Some(43));
    }
}
