pub struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<Vec<i32>>> = Vec::with_capacity(nums.len());
        nums.iter().rev().enumerate().for_each(|(idx, n)| {
            ret.push(Vec::new());
            nums.iter()
                .rev()
                .enumerate()
                .take(idx)
                .for_each(|(jdx, m)| {
                    if n <= m {
                        let mut tmp = Vec::new();
                        tmp.push(vec![*n, *m]);
                        ret[jdx].iter().for_each(|seq| {
                            let mut ttmmpp = vec![*n];
                            ttmmpp.extend_from_slice(&seq);
                            tmp.push(ttmmpp);
                        });
                        ret.last_mut().unwrap().extend_from_slice(&tmp);
                    }
                });
        });
        let ret: std::collections::HashSet<Vec<i32>> = ret
            .into_iter()
            .flatten()
            .filter(|seq| seq.len() > 0)
            .collect();
        ret.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let vec = vec![4, 6, 7, 7];
        let answer: std::collections::BTreeSet<Vec<i32>> = vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ]
        .into_iter()
        .collect();
        let my_response = Solution::find_subsequences(vec).into_iter().collect();
        let sym_diff: Vec<_> = answer.symmetric_difference(&my_response).collect();
        assert_eq!(sym_diff.len(), 0);

        let vec = vec![4, 4, 3, 2, 1];
        let answer: std::collections::BTreeSet<Vec<i32>> = vec![vec![4, 4]].into_iter().collect();
        let my_response = Solution::find_subsequences(vec).into_iter().collect();
        let sym_diff: Vec<_> = answer.symmetric_difference(&my_response).collect();
        assert_eq!(sym_diff.len(), 0);
    }
}
