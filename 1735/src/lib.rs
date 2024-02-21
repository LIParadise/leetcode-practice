pub struct Solution;

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        /*
         * 1. Prime factorization (with multiplicity)
         * 2. Supp. one factor is p^m, array has n slots,
         *    then it forms a nHm problem
         *    where nHm denotes # of non-negative solutions
         *    to the equation
         *    x_1 + x_2 + ... + x_n = m
         * 3. Mutliply them together, with possible overflow in mind.
         */

        let mut ret: Vec<i32> = Vec::new();
        let mut primes = Vec::new();
        match queries.iter().flatten().max() {
            None => {}
            Some(&ncr_max) => {
                let ncr = Self::make_ncr(std::cmp::min(ncr_max as usize * 2 - 1, 10012));
                // input guaranteed to be less than 10_000 by leetcode
                // multiplicity of primes is at most 2^13 = 8192
                // using nHr, we need at most mCs where m = 10000 + 13 - 1 = 10012
                queries.iter().for_each(|q| {
                    // Eratosthenes, preparing for prime factorization
                    let mut num = q[1] as usize;
                    Self::extend_primes(&mut primes, num);
                    // Prime factorization
                    let multiplicity: Vec<usize> = primes.iter().fold(Vec::new(), |mut m, p| {
                        let mut cnt = 0;
                        while num % p == 0 {
                            num /= p;
                            cnt += 1;
                        }
                        if cnt != 0 {
                            m.push(cnt);
                        }
                        m
                    });
                    ret.push(multiplicity.into_iter().fold(1, |acc, m| {
                        Self::mod_p(
                            acc as u64
                                * *Self::access_ncr(q[0] as usize + m - 1, m, &ncr).unwrap() as u64,
                        )
                    }) as i32);
                });
            }
        }
        ret
    }

    fn make_ncr(s: usize) -> Vec<Vec<u32>> {
        // Pascal's Triangle
        // 1
        // 1 1
        // 1 2 1
        // 1 3 3 1
        // 1 4 6 4 1
        // save only first half with prime modulo arithmetics
        let mut ret = Vec::with_capacity(s + 1);
        (0..=s).for_each(|i| ret.push(Vec::with_capacity(i / 2 + 1)));
        ret[0].push(1);
        (1..=s).for_each(|i| {
            let (known, working) = ret.split_at_mut(i);
            working.first_mut().unwrap().push(1);
            working.first_mut().unwrap().extend((1..i / 2 + 1).map(|j| {
                match known.last().unwrap().get(j) {
                    Some(k) => k + known.last().unwrap()[j - 1],
                    None => known.last().unwrap()[j - 1] * 2,
                }
            }));
            working
                .first_mut()
                .unwrap()
                .iter_mut()
                .for_each(|i| *i = Self::mod_p(*i as u64));
        });
        ret
    }

    fn access_ncr(n: usize, r: usize, ncr: &Vec<Vec<u32>>) -> Option<&u32> {
        let r = if r < n / 2 + 1 { r } else { n - r };
        ncr.get(n).map(|v| v.get(r)).flatten()
    }

    fn extend_primes(known_primes: &mut Vec<usize>, tgt: usize) {
        match known_primes.last() {
            None => {
                known_primes.extend(vec![2, 3, 5].into_iter());
                Self::extend_primes(known_primes, tgt);
            }
            Some(&known_max_p) => {
                if tgt > known_max_p {
                    let mut tmp = vec![true; tgt - known_max_p];
                    // known_primes: e.g. [2, 3, 5, 7, 11, ...]
                    // tmp: flags for contiguous int right after known_max_p
                    //
                    // supp. known_primes is [2, 3, 5, 7, 11]
                    // tmp stores flags for [12, 13, 14, ...]
                    // true denotes prime, false means composite
                    //
                    // tmp[i] stores the flag for (i + known_max_p + 1)
                    known_primes.iter().for_each(|&p| {
                        tmp.iter_mut()
                            .enumerate()
                            .map(|(i, b)| (i + known_max_p + 1, b))
                            .for_each(|(n, b)| {
                                if n % p == 0 {
                                    *b = false;
                                }
                            })
                    });
                    // supp. p is prime, Eratosthenes sieve goes from p*p
                    // note the correspondance between idx and underlying int
                    (known_max_p * (known_max_p - 1)..tmp.len())
                        .take_while(|i| (tgt / (i + known_max_p + 1)) <= (i + known_max_p + 1))
                        .for_each(|i| {
                            if tmp[i] {
                                (i..tmp.len())
                                    .step_by(i + known_max_p + 1)
                                    .skip(1)
                                    .for_each(|p| {
                                        tmp[p] = false;
                                    })
                            }
                        });
                    known_primes.extend(
                        tmp.into_iter()
                            .enumerate()
                            .filter_map(|(i, b)| b.then_some(i + known_max_p + 1)),
                    );
                }
            }
        }
    }

    fn mod_p(i: u64) -> u32 {
        static P: u64 = 1_000_000_007;
        (i % P) as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::ways_to_fill_array(vec![vec![2, 6], vec![5, 1], vec![73, 660]]),
            vec![4, 1, 50734910]
        );
        assert_eq!(
            Solution::ways_to_fill_array(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5],
                vec![10, 1],
                vec![10, 5],
                vec![1, 60]
            ]),
            vec![1, 2, 3, 10, 5, 1, 10, 1]
        );
        assert_eq!(
            Solution::ways_to_fill_array(vec![vec![3360, 1536], vec![6850, 6227]]),
            vec![921300655, 46922500]
        );
    }

    #[test]
    fn test_comb() {
        let ncr = Solution::make_ncr(5);
        assert_eq!(Solution::access_ncr(0, 0, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(1, 0, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(1, 1, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(2, 0, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(2, 1, &ncr), Some(&2));
        assert_eq!(Solution::access_ncr(2, 2, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(3, 0, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(3, 1, &ncr), Some(&3));
        assert_eq!(Solution::access_ncr(3, 2, &ncr), Some(&3));
        assert_eq!(Solution::access_ncr(3, 3, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(4, 0, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(4, 1, &ncr), Some(&4));
        assert_eq!(Solution::access_ncr(4, 2, &ncr), Some(&6));
        assert_eq!(Solution::access_ncr(4, 3, &ncr), Some(&4));
        assert_eq!(Solution::access_ncr(4, 4, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(5, 0, &ncr), Some(&1));
        assert_eq!(Solution::access_ncr(5, 1, &ncr), Some(&5));
        assert_eq!(Solution::access_ncr(5, 2, &ncr), Some(&10));
        assert_eq!(Solution::access_ncr(5, 3, &ncr), Some(&10));
        assert_eq!(Solution::access_ncr(5, 4, &ncr), Some(&5));
        assert_eq!(Solution::access_ncr(5, 5, &ncr), Some(&1));
    }

    #[test]
    fn test_eratosthenes() {
        let mut arr = Vec::new();
        Solution::extend_primes(&mut arr, 10);
        assert_eq!(arr, vec![2, 3, 5, 7]);
        Solution::extend_primes(&mut arr, 12);
        assert_eq!(arr, vec![2, 3, 5, 7, 11]);
        Solution::extend_primes(&mut arr, 12);
        assert_eq!(arr, vec![2, 3, 5, 7, 11]);
        Solution::extend_primes(&mut arr, 35);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
        Solution::extend_primes(&mut arr, 36);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
        Solution::extend_primes(&mut arr, 37);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
        let mut arr = Vec::new();
        Solution::extend_primes(&mut arr, 37);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
        let mut arr = Vec::new();
        Solution::extend_primes(&mut arr, 40);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
        Solution::extend_primes(&mut arr, 40);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
        Solution::extend_primes(&mut arr, 41);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]);
        Solution::extend_primes(&mut arr, 41);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]);
        Solution::extend_primes(&mut arr, 42);
        assert_eq!(arr, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]);
    }
}
