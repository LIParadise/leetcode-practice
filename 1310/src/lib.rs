pub struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = arr;
        (1..arr.len()).for_each(|i| {
            let (prev, next) = arr[..].split_at_mut(i);
            next.first_mut().map(|n| *n = *n ^ prev.last().unwrap());
        });
        let mut ret = Vec::with_capacity(arr.len());
        queries
            .iter()
            .map(|v| {
                (
                    v.get(0).unwrap().clone() as usize,
                    v.get(1).unwrap().clone() as usize,
                )
            })
            .for_each(|(i, j)| {
                ret.push(if i == 0 { arr[j] } else { arr[i - 1] ^ arr[j] });
            });
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        assert_eq!(Solution::xor_queries(arr, queries), vec![2, 7, 14, 8]);
    }
}
