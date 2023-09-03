pub struct Solution;

trait UpsideDownTriangleIndexingTo1D {
    // Assuming self be a 1D array, conceptually we have a upside-down triangle.
    // E.g.
    // 5 4 3 2 1
    // 6 7 8 9
    // 3 2 1
    // 2 1
    // 1
    // s.t. [5, 4, 3, 2, 1, 6, 7, 8, 9, 3, 2, 1, 2, 1, 1]
    // Input (1, 3), we need to get 9.
    fn get_2d<I, T>(&self, i: usize, j: usize, n: usize) -> Option<&I::Output>
    where
        I: std::slice::SliceIndex<[T], Output = T>,
        Self: std::ops::Deref<Target = [T]>,
    {
        // to prevent overflow, determine odd/even first
        if i % 2 == 0 {
            self.get((i / 2) * (2 * n - i + 1) + j)
        } else {
            self.get(i * ((2 * n - i + 1) / 2) + j)
        }
    }
    fn get_mut_2d<I, T>(&mut self, i: usize, j: usize, n: usize) -> Option<&mut I::Output>
    where
        I: std::slice::SliceIndex<[T], Output = T>,
        Self: std::ops::DerefMut<Target = [T]>,
    {
        // to prevent overflow, determine odd/even first
        if i % 2 == 0 {
            self.get_mut((i / 2) * (2 * n - i + 1) + j)
        } else {
            self.get_mut(i * ((2 * n - i + 1) / 2) + j)
        }
    }
}

impl<T> UpsideDownTriangleIndexingTo1D for Vec<T> where T: Sized {}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        // prevent, slightly, from overflow
        let dp_arr_len = if piles.len() % 2 == 0 {
            piles.len() / 2 * (piles.len() + 1)
        } else {
            (piles.len() + 1) / 2 * piles.len()
        };
        let mut dp_arr = Vec::with_capacity(dp_arr_len);
        (0..dp_arr_len).for_each(|_| dp_arr.push(None));

        (1..=piles.len()).for_each(|interval_len| {
            (0..piles.len() - (interval_len - 1)).for_each(|start_idx| {
                // Store game result for slice [i..=j] in this iteration
                // `tuple.0` is for the first guy, `tuple.1` is for the second
                let i = start_idx;
                let j = start_idx + (interval_len - 1);

                if interval_len == 1 {
                    if let Some(ptr) =
                        dp_arr.get_mut_2d::<usize, _>(interval_len - 1, start_idx, piles.len())
                    {
                        *ptr = Some((piles.get(i).unwrap().to_owned(), 0_i32));
                    }
                } else {
                    // FIXME
                    // why need type annotation here?
                    let left_subarr_ans: (i32, i32) = dp_arr
                        .get_2d::<usize, _>(interval_len - 2, i, piles.len())
                        .unwrap()
                        .unwrap();
                    let right_subarr_ans = dp_arr
                        .get_2d::<usize, _>(interval_len - 2, i + 1, piles.len())
                        .unwrap()
                        .unwrap();
                    let take_right = piles.get(j).unwrap() + left_subarr_ans.1;
                    let take_left = piles.get(i).unwrap() + right_subarr_ans.1;
                    if let Some(ptr) =
                        dp_arr.get_mut_2d::<usize, _>(interval_len - 1, i, piles.len())
                    {
                        *ptr = if take_left > take_right {
                            Some((take_left, right_subarr_ans.0))
                        } else {
                            Some((take_right, left_subarr_ans.0))
                        }
                    }
                }
            })
        });
        let game_result = dp_arr
            .get_2d::<usize, _>(piles.len() - 1, 0, piles.len())
            .unwrap()
            .unwrap();
        game_result.0 >= game_result.1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::UpsideDownTriangleIndexingTo1D as tri_to_flat;
    #[test]
    fn test_soln() {
        let piles = vec![5, 3, 4, 5];
        assert!(Solution::stone_game(piles));
        let piles = vec![3, 7, 2, 3];
        assert!(Solution::stone_game(piles));
    }

    #[test]
    fn test_tri_2d_to_1d() {
        let vec = vec![5, 4, 3, 2, 1, 9, 8, 7, 6, 12, 11, 10, 14, 13, 15];
        // Why do we need type annotation here?
        assert_eq!(vec.get_2d::<usize, _>(0, 4, 5), Some(&1));
        assert_eq!(vec.get_2d::<usize, _>(1, 2, 5), Some(&7));
        assert_eq!(vec.get_2d::<usize, _>(4, 0, 5), Some(&15));
    }
}
