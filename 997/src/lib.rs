pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut out_degree = vec![false; n as usize];
        let mut in_degree = vec![0_usize; n as usize];
        trust.iter().for_each(|v| {
            out_degree[v[0] as usize - 1] = true;
            in_degree[v[1] as usize - 1] += 1;
        });
        println!("{:?}", out_degree);
        out_degree
            .iter()
            .zip(in_degree.iter().enumerate())
            .find_map(|(&b, (u, &cnt))| {
                if !b && cnt == n as usize - 1 {
                    Some(u as i32 + 1)
                } else {
                    None
                }
            })
            .map_or(-1, |x| x)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
    }
}
