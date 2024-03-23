pub struct Solution;

impl Solution {
    /// Consider array of length 2 with partial order defined as following:
    /// a > b iff (a[0] > b[0]) && (a[1] > b[1])
    ///
    /// Given array of arrays of length 2,
    /// return # of elements that are not maximal
    // O(n^2) brute force not possible since input length spec goes up to 10^5
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        let mut counter = 0;
        while !properties.is_empty() {
            let maximal = properties
                .iter()
                .reduce(|maximal, n| if Self::lt(maximal, n) { n } else { maximal })
                .cloned()
                .unwrap();
            loop {
                let new_maximal = properties.iter().fold(&maximal, |maximal, n| {
                    if Self::lt(&maximal, n) {
                        n
                    } else {
                        &maximal
                    }
                });
                if *new_maximal == maximal {
                    break;
                }
            }
            let mut last_idx = 0;
            while let Some(idx) = properties[last_idx..]
                .iter()
                .position(|n| Self::lt(n, &maximal) || n == &maximal)
            {
                last_idx = idx + last_idx;
                if Self::lt(&properties[last_idx], &maximal) {
                    counter += 1;
                }
                properties.swap_remove(last_idx);
            }
        }
        counter
    }
    fn lt(a: &[i32], b: &[i32]) -> bool {
        a.iter().zip(b.iter()).all(|(a, b)| a < b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
            0
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![3, 3], vec![2, 2]]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
            1
        );
    }
}
