pub struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes = vec![2, 3, 5, 7_i32];
        (11..=right).step_by(2).for_each(|i| {
            match primes
                .iter()
                .take_while(|p| *p * *p <= i)
                .try_for_each(|p| (0 != i % p).then(|| ()))
            {
                None => {}
                Some(()) => primes.push(i),
            }
        });
        let l_idx = primes.partition_point(|&p| p < left);
        let r_idx = primes.partition_point(|&p| p <= right);
        match primes[l_idx..r_idx]
            .iter()
            .zip(primes[l_idx..r_idx].iter().skip(1))
            .reduce(|(q1, q2), (p1, p2)| {
                if p2 - p1 < q2 - q1 {
                    (p1, p2)
                } else {
                    (q1, q2)
                }
            }) {
            None => vec![-1, -1],
            Some((&p1, &p2)) => vec![p1, p2],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::closest_primes(10, 19), vec![11, 13]);
        assert_eq!(Solution::closest_primes(4, 6), vec![-1, -1]);
    }
}
