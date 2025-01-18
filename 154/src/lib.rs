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
                    // the tricky part
                    // though left, mid, and right are equal,
                    // left just might not be the real min,
                    // e.g. `[0, -1, 0, 0, 0]` or `[0, 0, 0, -1, 0]`
                    while arr[right] == arr[left] && left < right {
                        right -= 1;
                    }
                    if arr[right] > arr[left] {
                        return left;
                    } else if arr[right] < arr[left] {
                        left += 1;
                    }
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

