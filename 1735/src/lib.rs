pub struct Solution;

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        /*
         * 1. find prime factorization
         * 2. supp. n primes, for m = 1 to n, merge primes (C(n,m) of them)
         *    each merge contributes exactly the same # of conbinations
         *    remember to pad 1
         * 3. sum these
         */
        let mut ret: Vec<i32> = Vec::new();
        let mut primes = Vec::new();
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
                Self::mod_p(Self::mod_p(Self::nhr(q[0] as usize, m)) * acc)
            }) as i32);
        });
        ret
    }

    fn extend_primes(known_primes: &mut Vec<usize>, tgt: usize) {
        match known_primes.last() {
            None => {
                *known_primes = vec![2, 3, 5];
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
                    // tmp[i] stores the flag for (known_max_p + 1 + i)
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
                                    .step_by(known_max_p + i + 1)
                                    .skip(1)
                                    .for_each(|p| {
                                        tmp[p] = false;
                                    })
                            }
                        });
                    tmp.into_iter()
                        .enumerate()
                        .filter_map(|(i, b)| b.then_some(i))
                        .for_each(|i| {
                            known_primes.push(i + known_max_p + 1);
                        });
                }
            }
        }
    }

    fn mod_p(i: usize) -> usize {
        static P: usize = 1_000_000_007;
        i % P
    }

    /// Simple nCr implementation
    /// Assuming non-negative semantics
    fn ncr(n: usize, r: usize) -> usize {
        if r > n {
            0
        } else {
            (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
        }
    }

    /// # of non-negative integer solutions to the following?
    /// x_1 + x_2 + ... + x_n = r
    /// answer is nHr or nhr(n,r)
    fn nhr(n: usize, r: usize) -> usize {
        Self::ncr(n + r - 1, r)
    }

    fn extend_num_of_prime_factors(arr: &mut Vec<i32>, tgt: usize) {
        let old_len = arr.len();
        if tgt >= arr.len() {
            let mut new = Vec::with_capacity(tgt - old_len + 1);
            (0..tgt - old_len + 1).for_each(|_| new.push(1));
            arr.iter()
                .enumerate()
                .filter(|(_, &cnt)| cnt == 1)
                .for_each(|(p, _)| {
                    (std::cmp::max((1 + (old_len - 1) / p) * p, p * p)..=tgt)
                        .step_by(p)
                        .for_each(|i| {
                            new[i - old_len] = 1 + if i / p < old_len {
                                arr[i / p]
                            } else {
                                new[i / p - old_len]
                            };
                        });
                });
            arr.extend(new.into_iter());
            let mut search_after_here = old_len - 1;
            while let Some(p) = arr
                .iter()
                .enumerate()
                .skip(search_after_here + 1)
                .find(|(_, &cnt)| cnt == 1)
                .map(|(p, _)| p)
            {
                search_after_here = p;
                if (arr.len() - 1) / p < p {
                    break;
                }
                (p * p..arr.len()).step_by(p).for_each(|i| {
                    let (left, right) = arr.split_at_mut(i);
                    if let Some(1) = right.first() {
                        *right.first_mut().unwrap() = left[i / p] + 1;
                    }
                });
            }
        }
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
                vec![5, 5]
            ]),
            vec![1, 2, 3, 10, 5]
        );
        assert_eq!(
            Solution::ways_to_fill_array(vec![vec![3360, 1536], vec![6850, 6227]]),
            vec![921300655, 46922500]
        );
    }

    #[test]
    fn test_extend_num_of_prime_factors() {
        let mut arr = vec![0, 0, 1, 1];
        Solution::extend_num_of_prime_factors(&mut arr, 21);
        assert_eq!(
            vec![0, 0, 1, 1, 2, 1, 2, 1, 3, 2, 2, 1, 3, 1, 2, 2, 4, 1, 3, 1, 3, 2],
            arr
        );
        /*
         * [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]
         *        2  3  2  5  2  7  2  3   2  11   2  13   2   3   2  17   2  19   2   3
         *              2     3     2  3   5       2       7   5   2       3       2   7
         *                          2              3               2       3       5
         *                                                         2
         * [0, 0, 1, 1, 2, 1, 2, 1, 3, 2,  2,  1,  3,  1,  2,  2,  4,  1,  3,  1,  3,  2]
         */
        let mut arr = vec![0, 0];
        Solution::extend_num_of_prime_factors(&mut arr, 78);
        /*
         * [OEIS](https://oeis.org/wiki/Prime_factors)
         * [0, 0, 1, 1, 2, 1, 2, 1, 3, 2, 2, 1, 3, 1, 2, 2, 4, 1, 3, 1, 3, 2, 2, 1, 4, 2, 2, 3, 3, 1,
         *  3, 1, 5, 2, 2, 2, 4, 1, 2, 2, 4, 1, 3, 1, 3, 3, 2, 1, 5, 2, 3, 2, 3, 1, 4, 2, 4, 2, 2,
         *  1, 4, 1, 2, 3, 6, 2, 3, 1, 3, 2, 3, 1, 5, 1, 2, 3, 3, 2, 3]
         * Notice here I start with 0 (TBH kinda ill-formed) instead of 1.
         * */
    }

    #[test]
    fn test_comb() {
        assert_eq!(Solution::ncr(5, 3), Solution::ncr(5, 2));
        assert_eq!(Solution::ncr(10, 4), 210);
        assert_eq!(Solution::nhr(3, 2), 6);
        assert_eq!(Solution::nhr(5, 1), 5);
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
