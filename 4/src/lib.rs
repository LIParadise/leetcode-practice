pub struct Solution;

impl Solution {
    fn virt_mid_idx(alpha_1: usize, omega_1: usize, alpha_2: usize, omega_2: usize) -> Vec<usize> {
        let len_1 = omega_1 - alpha_1;
        let len_2 = omega_2 - alpha_2;
        let two_medians = (len_1 % 2) == (len_2 % 2);
        if two_medians {
            // Supp. len 50
            // [0, ..., 24, 25, ..., 49]
            // Want vec![24, 25]
            if len_1 % 2 == 1 {
                // Both odd
                // e.g. 23 and 27
                //      11  +  13 = 24
                vec![len_1 / 2 + len_2 / 2, len_1 / 2 + len_2 / 2 + 1]
            } else {
                // Both even
                // e.g. 24 and 26
                //      12  +  13 = 25
                vec![len_1 / 2 + len_2 / 2 - 1, len_1 / 2 + len_2 / 2]
            }
        } else {
            // Supp. len 51
            // [0, ..., 25, ..., 50]
            // Want vec![25]
            // e.g. 23 and 28
            //      11  +  14 = 25
            vec![len_1 / 2 + len_2 / 2]
        }
    }
    fn find_median_sorted_array<
        T: PartialEq
            + PartialOrd
            + Eq
            + Ord
            + std::ops::Add<Output = T>
            + Into<f64>
            + From<u8>
            + Clone
            + std::ops::Div<Output = T>,
    >(
        vec: &[T],
    ) -> f64 {
        if vec.len() == 0 {
            panic!("That there exists an element in a null set is false.");
        }
        if vec.len() % 2 == 1 {
            vec[vec.len() / 2].clone().into()
        } else {
            <T as Into<f64>>::into(vec[vec.len() / 2].clone() + vec[vec.len() / 2 - 1].clone())
                / 2.0
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 && nums2.len() == 0 {
            panic!("That there exists an element in a null set is false.");
        } else if nums1.len() == 0 {
            return Self::find_median_sorted_array(&nums2[..]);
        } else if nums2.len() == 0 {
            return Self::find_median_sorted_array(&nums1[..]);
        }

        // Both have non-zero length
        let (mut alpha_1, mut alpha_2, mut omega_1, mut omega_2) =
            (0_usize, 0_usize, nums1.len(), nums2.len());
        let mut tgt_rank_idx = Self::virt_mid_idx(alpha_1, omega_1, alpha_2, omega_2);
        let mut virt_in_1;
        let mut medians = Vec::new();
        loop {
            let virt_arr_mid = Self::virt_mid_idx(alpha_1, omega_1, alpha_2, omega_2)
                .first()
                .expect("Unexpected error when find virtual mid")
                .clone();
            let (virt_this_arr, virt_other_arr, virt_this_idx) = if virt_arr_mid < omega_1 - alpha_1
            {
                virt_in_1 = true;
                (
                    nums1.get(alpha_1..omega_1).unwrap(),
                    nums2.get(alpha_2..omega_2).unwrap(),
                    nums1
                        .get(alpha_1..omega_1)
                        .unwrap()
                        .partition_point(|i| i <= &nums1[alpha_1 + virt_arr_mid])
                        - 1,
                )
            } else {
                virt_in_1 = false;
                (
                    nums2.get(alpha_2..omega_2).unwrap(),
                    nums1.get(alpha_1..omega_1).unwrap(),
                    nums2.get(alpha_2..omega_2).unwrap().partition_point(|i| {
                        i <= &nums2[alpha_2 + virt_arr_mid + alpha_1 - omega_1]
                    }) - 1,
                )
            };

            let virt_rank_idx = virt_this_idx
                + virt_other_arr.partition_point(|i| i <= &virt_this_arr[virt_this_idx]);
            let rank_idx = virt_rank_idx + alpha_1 + alpha_2;
            if tgt_rank_idx.contains(&rank_idx) {
                medians.push(virt_this_arr[virt_this_idx]);
                tgt_rank_idx.retain(|i| i != &rank_idx);
                if tgt_rank_idx.is_empty() {
                    break;
                }
            } else {
                if tgt_rank_idx.iter().all(|i| i < &rank_idx) {
                    if virt_in_1 {
                        omega_2 = alpha_2 + virt_rank_idx - virt_this_idx;
                        omega_1 = alpha_1 + virt_this_idx;
                    } else {
                        omega_1 = alpha_1 + virt_rank_idx - virt_this_idx;
                        omega_2 = alpha_2 + virt_this_idx;
                    }
                } else if tgt_rank_idx.iter().all(|i| i > &rank_idx) {
                    if virt_in_1 {
                        alpha_2 = alpha_2 + virt_rank_idx - virt_this_idx;
                        alpha_1 = alpha_1 + virt_this_idx + 1;
                    } else {
                        alpha_1 = alpha_1 + virt_rank_idx - virt_this_idx;
                        alpha_2 = alpha_2 + virt_this_idx + 1;
                    }
                } else {
                    panic!("Unknown index");
                }
            }
        }
        (medians.iter().sum::<i32>() as f64) / (medians.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let output = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(output, 2.0);
        let output = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(output, 2.5);
    }
}
