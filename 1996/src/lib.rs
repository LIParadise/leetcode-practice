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
            let pivot = properties.first().cloned().unwrap();
            while let Some(smaller) = properties
                .iter()
                .position(|smaller| Self::lt(smaller, &pivot))
            {
                properties.swap_remove(smaller);
                counter += 1;
            }
            let pivot_is_maximal = if let Some(_) = properties
                .iter()
                .position(|greater| Self::lt(&pivot, greater))
            {
                false
            } else {
                true
            };
            while let Some(equal) = properties.iter().position(|p| p == &pivot) {
                properties.swap_remove(equal);
                if !pivot_is_maximal {
                    counter += 1;
                }
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
