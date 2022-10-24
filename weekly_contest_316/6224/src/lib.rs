use std::ops::Deref;

pub struct Solution;

#[derive(Debug)]
pub struct Primes {
    primes: Vec<usize>,
}
impl Deref for Primes {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.primes
    }
}
impl Primes {
    pub fn new() -> Self {
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
        ];
        Self { primes }
    }
    pub fn check(&mut self, tgt: usize) -> bool {
        let mut idx = self.primes.partition_point(|n| n < &tgt);
        if idx >= self.primes.len() {
            let mut eratos_index_end = self
                .primes
                .partition_point(|n| n * n <= self.primes.last().unwrap().clone());
            for possible_prime in (self.primes.last().unwrap().clone() + 2..=tgt).step_by(2) {
                while self.primes[eratos_index_end] * self.primes[eratos_index_end]
                    <= possible_prime
                {
                    eratos_index_end += 1;
                }
                if self
                    .primes
                    .get(..eratos_index_end)
                    .unwrap()
                    .iter()
                    .all(|&p| possible_prime % p != 0)
                {
                    self.primes.push(possible_prime)
                }
            }
            idx = self.primes.partition_point(|n| n < &tgt);
        }
        match self.primes.get(idx) {
            Some(p) if p == &tgt => true,
            _ => false,
        }
    }
}

impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(|n| n as usize).collect::<Vec<_>>();
        let k = k as usize;
        let mut primes = Primes::new();
        let mut factors: Vec<std::collections::BTreeSet<usize>> = Vec::new();
        nums.iter().for_each(|&n| {
            if n >= k && n % k == 0 {
                let quotient = n / k;
                let mut factors_of_quotient = std::collections::BTreeSet::from([1; 1]);
                if primes.check(quotient) {
                    factors_of_quotient.insert(quotient);
                } else {
                    primes
                        .iter()
                        .take_while(|&&p| p * p <= quotient)
                        .for_each(|&p| {
                            if quotient % p == 0 {
                                factors_of_quotient.insert(p);
                            }
                        })
                }
                factors.push(factors_of_quotient);
            } else {
                factors.push(std::collections::BTreeSet::new());
            }
        });
        let mut ret = 0;
        for start in 0..nums.len() {
            for end in start + 1..nums.len() + 1 {
                let mut common_factors = factors[start].clone();
                for i in start + 1..end {
                    if factors[i].is_empty() {
                        common_factors.clear();
                    }
                    if common_factors.is_empty() {
                        break;
                    }
                    common_factors = common_factors
                        .intersection(&factors[i])
                        .map(|u| u.clone())
                        .collect();
                }
                if common_factors.len() == 1 {
                    ret += 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::{Primes, Solution};
    #[test]
    fn test_soln() {
        assert_eq!(Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
        assert_eq!(Solution::subarray_gcd(vec![4], 7), 0);
    }
}
