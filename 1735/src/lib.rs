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
        let mut num_of_prime_factors = vec![0, 0, 1, 1];
        Self::extend_num_of_prime_factors(&mut num_of_prime_factors, 1009);
        let arr = &mut num_of_prime_factors;
        let mut ret = Vec::with_capacity(queries.len());
        match queries.iter().map(|query| query[1] as usize).max() {
            Some(l) => Self::extend_num_of_prime_factors(arr, l),
            None => return Vec::new(),
        }
        queries.iter().for_each(|query| {
            let slots = query[0];
            let prod = query[1];
            let num_factors = arr[prod as usize];
            let mut cnt = 0;
            if prod == 1 {
                cnt = 1
            } else {
                (1..=std::cmp::min(slots, num_factors)).for_each(|merge_some| {
                    // consider the factors merged to a total of the iterator
                    // e.g. start by 1, meaning merge all, i.e. the product itself
                    // The other unfilled entries are padded with 1
                    // Such merge has (n, m) kinds.
                    let sub_cnt = Self::mod_p(Self::ncr(num_factors, merge_some))
                        * Self::mod_p(Self::ncr(slots, merge_some))
                        * (1..=merge_some).product::<i32>();
                    cnt = Self::mod_p(cnt + sub_cnt);
                });
            }
            ret.push(cnt);
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

    fn mod_p(i: i32) -> i32 {
        static P: i32 = 1_000_000_007;
        i % P
    }

    /// Simple nCr implementation
    /// Assuming non-negative semantics
    fn ncr(n: i32, r: i32) -> i32 {
        if r > n {
            0
        } else {
            (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
        }
    }

    /// # of non-negative integer solutions to the following?
    /// x_1 + x_2 + ... + x_n = r
    /// answer is nhr(n,r)
    fn nhr(n: i32, r: i32) -> i32 {
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
    }
}
