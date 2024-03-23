pub struct Solution;

impl Solution {
    /// Consider array of length 2 with partial order defined as following:
    /// a > b iff (a[0] > b[0]) && (a[1] > b[1])
    ///
    /// Given array of arrays of length 2,
    /// return # of elements that are not maximal
    // O(n^2) brute force not possible since input length spec goes up to 10^5
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        // Hoare's partition (ref: quicksort variants)
        //
        // Lemma:
        // One and only one occurs between a and b:
        // 1. a > b
        // 2. a < b
        // 3. a and b are not comparable
        // Proof:
        // supp. a and b are element-wise equal, they are not comparable under this certain
        // partial ordering.
        //
        // Idea:
        // Partition input s.t. those smaller than pivot are at the end; discard them.

        let mut counter = 0;
        loop {
            if let Some(pivot) = properties.first().cloned() {
                let mut l_idx = 0;
                let mut r_idx = properties.len() - 1;
                let mut pivot_is_maximal = true;
                loop {
                    while let Some(l) = properties.get(l_idx) {
                        if Self::lt(&pivot, l) || !Self::comparable(&pivot, l) {
                            // notice pivot is chosen to be the first guy,
                            // and it just stays there
                            // which serves also as boundary for `r_idx`
                            if Self::lt(&pivot, l) {
                                pivot_is_maximal = false;
                            }
                            l_idx += 1;
                        } else {
                            break;
                        }
                    }
                    while Self::lt(&properties[r_idx], &pivot) {
                        r_idx -= 1;
                    }
                    if Self::lt(&pivot, &properties[r_idx]) {
                        pivot_is_maximal = false;
                    }
                    if l_idx == r_idx {
                        panic!("Unexpected Hoare partitioning with custom partial order");
                    } else if l_idx > r_idx {
                        break;
                    } else {
                        properties.swap(l_idx, r_idx);
                        l_idx += 1;
                        r_idx -= 1;
                    }
                }
                counter += properties.len() - l_idx;
                properties.resize(l_idx, Vec::new());
                while let Some(pivot) = properties.iter().position(|v| *v == pivot) {
                    properties.swap_remove(pivot);
                    if !pivot_is_maximal {
                        counter += 1;
                    }
                }
            } else {
                break;
            }
        }
        counter.try_into().unwrap()
    }
    fn lt(a: &[i32], b: &[i32]) -> bool {
        a.iter().zip(b.iter()).all(|(a, b)| a < b)
    }
    fn comparable(a: &[i32], b: &[i32]) -> bool {
        Self::lt(a, b) || Self::lt(b, a)
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
