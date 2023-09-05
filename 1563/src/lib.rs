pub struct Solution;

struct TriangleArray<T: Sized> {
    diag: usize,
    pub vec: Vec<T>,
}

impl<T> TriangleArray<T> {
    pub fn new(diag: usize) -> Self {
        let len = if diag % 2 == 0 {
            diag / 2 * (diag + 1)
        } else {
            (diag + 1) / 2 * diag
        };
        let vec = Vec::with_capacity(len);
        Self { diag, vec }
    }
    fn get_idx(i: usize, j: usize, diag: usize) -> usize {
        (if i % 2 == 0 {
            i / 2 * (diag + diag + 1 - i)
        } else {
            (diag + diag + 1 - i) / 2 * i
        }) + j
    }
    pub fn get_2d(&self, i: usize, j: usize) -> Option<&T> {
        self.vec.get(Self::get_idx(i, j, self.diag))
    }
}

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let mut dp_values: TriangleArray<i32> = TriangleArray::new(stone_value.len());
        let mut sum_subslices: TriangleArray<i32> = TriangleArray::new(stone_value.len());
        let mut dp_choices: TriangleArray<usize> = TriangleArray::new(stone_value.len() - 1);
        // O(n^3)
        (1..=stone_value.len()).for_each(|slice_len| {
            (0..stone_value.len() - (slice_len - 1)).for_each(|i| {
                if slice_len == 1 {
                    sum_subslices.vec.push(stone_value[i]);
                } else {
                    let j = i + slice_len;
                    if i == 0 {
                        sum_subslices.vec.push(
                            stone_value[j - 1] + sum_subslices.get_2d(slice_len - 2, i).unwrap(),
                        );
                    } else {
                        sum_subslices.vec.push(
                            sum_subslices.vec.last().unwrap() - stone_value[i - 1]
                                + stone_value[j - 1],
                        );
                    }
                }
            })
        });
        (1..=stone_value.len()).for_each(|slice_len| {
            (0..stone_value.len() - (slice_len - 1)).for_each(|i| {
                // considering subproblem for [i..j]
                if 1 == slice_len {
                    dp_values.vec.push(0);
                } else {
                    // [i..j]
                    let j = i + slice_len;
                    let max_min = (i + 1..j)
                        .fold(None, |max_min, k| {
                            // [i..j] => [i..k] union [k..j]
                            let left_slice_sum = sum_subslices.get_2d(k - i - 1, i).unwrap();
                            let left_slice_dp = dp_values.get_2d(k - i - 1, i).unwrap();
                            let right_slice_sum = sum_subslices.get_2d(j - k - 1, k).unwrap();
                            let right_slice_dp = dp_values.get_2d(j - k - 1, k).unwrap();
                            let left = left_slice_sum + left_slice_dp;
                            let right = right_slice_sum + right_slice_dp;
                            if let None = max_min {
                                if left_slice_sum >= right_slice_sum {
                                    Some((k, right))
                                } else {
                                    Some((k, left))
                                }
                            } else {
                                if left_slice_sum > right_slice_sum {
                                    if right >= max_min.unwrap().1 {
                                        Some((k, right))
                                    } else {
                                        max_min
                                    }
                                } else if left_slice_sum == right_slice_sum {
                                    let mut tmp = max_min;
                                    if right >= tmp.unwrap().1 {
                                        tmp = Some((k, right));
                                    }
                                    if left >= tmp.unwrap().1 {
                                        tmp = Some((k, left));
                                    }
                                    tmp
                                } else {
                                    if left >= max_min.unwrap().1 {
                                        Some((k, left))
                                    } else {
                                        max_min
                                    }
                                }
                            }
                        })
                        .unwrap();
                    dp_choices.vec.push(max_min.0);
                    dp_values.vec.push(max_min.1.clone());
                }
            })
        });
        dp_values.get_2d(stone_value.len() - 1, 0).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let stone_value = vec![6, 2, 3, 4, 5, 5];
        assert_eq!(Solution::stone_game_v(stone_value), 18);
        let stone_value = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::stone_game_v(stone_value), 28);
        let stone_value = vec![9, 8, 2, 4, 6, 3, 5, 1, 7];
        assert_eq!(Solution::stone_game_v(stone_value), 34);
        let stone_value = vec![1, 1, 2];
        assert_eq!(Solution::stone_game_v(stone_value), 3);
    }

    use crate::TriangleArray;
    #[test]
    fn test_trig_arr() {
        // 10 9 8 7
        // 1  2 3
        // 6  5
        // 4
        let arr = vec![10, 9, 8, 7, 1, 2, 3, 6, 5, 4];
        let mut triangle = TriangleArray::new(4);
        arr.iter().for_each(|a| triangle.vec.push(*a));
        assert_eq!(triangle.get_2d(0, 1), Some(&9));
        assert_eq!(triangle.get_2d(1, 2), Some(&3));
        assert_eq!(triangle.get_2d(3, 0), Some(&4));
    }
}
