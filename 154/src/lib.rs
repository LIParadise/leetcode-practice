pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        Self::rotated_sorted_array_get_min_idx(&nums)
            .and_then(|i| nums.get(i).cloned())
            .unwrap_or(i32::MIN)
    }
    fn rotated_sorted_array_get_min_idx(arr: &[i32]) -> Option<usize> {
        (!arr.is_empty()).then(|| {
            let mut left = 0;
            let mut right = arr.len() - 1;
            while left < right {
                let mid = Self::get_mid(left, right);
                if arr[mid] > arr[right] {
                    left = mid + 1;
                } else if arr[left] > arr[mid] {
                    left += 1;
                    right = mid;
                } else if arr[left] < arr[right] {
                    return left;
                } else if arr[left] == arr[right] {
                    /*
                     * the tricky part
                     * though left, mid, and right are equal,
                     * left just might not be the real min,
                     * e.g. `[0, -1, 0, 0, 0]` or `[0, 0, 0, -1, 0]`
                     *
                     * You know what?
                     * We can proof that this question is $O(n)$ i.e. linear time
                     *
                     * Adversary argument:
                     * https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/description/comments/1569484
                     *
                     * Suppose an algorithm A claims to beat worst case linear time,
                     * consider an array with length N,
                     * in which all elements are equal, say all are ones.
                     *
                     * The (deterministic) algorithm don't know this preliminary fact,
                     * so A accesses the array and try to determine the next entry it wants to see;
                     * being deterministic, there exists an index to which A last attempt to access,
                     * say it's I.
                     *
                     * Now consider an array of N numbers each of which are equal
                     * except at I it's one less than others.
                     *
                     * A must work out all N numbers in order to get the correct answer.
                     */
                    return arr[left..=right]
                        .iter()
                        .enumerate()
                        .fold((0, arr[left]), |(idx_of_min, min), (idx, &val)| {
                            if val < min {
                                (idx, val)
                            } else {
                                (idx_of_min, min)
                            }
                        })
                        .0
                        + left;
                }
            }
            left
        })
    }
    fn get_mid(a: usize, b: usize) -> usize {
        a.checked_add(b)
            .map(|sum| sum / 2)
            .unwrap_or_else(|| a / 2 + b / 2 + (a % 2 + b % 2) / 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let v = [];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), None);
        let v = [0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [0, 1];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [1, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(1));
        let v = [0, 1, 2];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [2, 0, 1];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(1));
        let v = [1, 2, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(2));

        let v = [0, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [0, 0, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [0, 1, 1];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [1, 0, 1];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(1));
        let v = [1, 1, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(2));
        let v = [0, 0, 1];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(0));
        let v = [1, 0, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(1));
        let v = [0, 1, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(2));

        let v = [0, -1, 0, 0, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(1));
        let v = [0, 0, 0, -1, 0];
        assert_eq!(Solution::rotated_sorted_array_get_min_idx(&v), Some(3));
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
