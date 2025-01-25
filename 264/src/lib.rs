pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    /*
     * Some failed thoughts:
     * 1. Maybe they are abundant enough s.t. generic array works as a nice hash?
     *    This line of thought naively assumes that
     *    $1/2 + 1/3 + 1/5 - 1/6 - 1/15 - 1/10 + 1/30$ is the frequency,
     *    yet it's simply not true:
     *    the line calculates the numbers that have **at least** one prime factor that's
     *    either 2, 3, or 5,
     *    but we want **all** the prime factors to be these three.
     * 2. So maybe 3 lists caching the numbers of which "last" factor is resp. 2, 3, or 5?
     *    It simply doesn't work this way:
     *    This algorithm would keep `[1, 2, 4, 6]`, `[1, 3]`, `[1, 5]`, yet the next is `8`,
     *    not calculated from the last guy of any of the list
     */
    /// An **ugly** number is number of which prime factors composes only of 2, 3, or 5.
    ///
    /// Lemma:
    /// suppose n is ugly, then 2n, 3n, and 5n are all ugly.
    ///
    /// We maintain 3 lists,
    /// one that all 2, 3, 5 times it all hasn't been considered,
    /// one that 3 and 5 are yet considered,
    /// and finally on that only 5 times it is yet considered.
    ///
    /// On each iteration, we compare 2 times the smallest guy in the first list,
    /// 3 times the smallest guy in the second list,
    /// and 5 times the smallest guy in the third list,
    /// push the smallest of these three to the first list,
    /// and "downgrade" the guy with which multiplied by some prime becomes the new guy.
    pub fn nth_ugly_number(n: i32) -> i32 {
        (1..n)
            .fold(
                (
                    1,
                    VecDeque::from_iter([1_usize]),
                    VecDeque::<usize>::new(),
                    VecDeque::<usize>::new(),
                ),
                |(_, mut yet_2_3_5, mut yet_3_5, mut yet_5), _| {
                    let mut ugly = yet_5.front().copied().map(|x| x * 5);
                    if let Some(x) = yet_3_5.front().copied().map(|x| x * 3) {
                        if ugly.is_none() || x < ugly.unwrap() {
                            ugly = Some(x);
                        }
                    }
                    if let Some(x) = yet_2_3_5.front().copied().map(|x| x * 2) {
                        if ugly.is_none() || x < ugly.unwrap() {
                            ugly = Some(x);
                        }
                    }
                    let ugly = ugly.unwrap();
                    match yet_5.front().copied().map(|x| x * 5) {
                        None => { /* no-op */ }
                        Some(x) if x < ugly => panic!(),
                        Some(x) if x == ugly => {
                            yet_5.pop_front();
                        }
                        Some(x) if x > ugly => { /* no-op for next use */ }
                        _ => unreachable!(),
                    }
                    match yet_3_5.front().copied().map(|x| x * 3) {
                        None => { /* no-op */ }
                        Some(x) if x < ugly => panic!(),
                        Some(x) if x == ugly => {
                            yet_5.extend(yet_3_5.pop_front());
                        }
                        Some(x) if x > ugly => { /* no-op for next use */ }
                        _ => unreachable!(),
                    }
                    match yet_2_3_5.front().copied().map(|x| x * 2) {
                        None => { /* no-op */ }
                        Some(x) if x < ugly => panic!(),
                        Some(x) if x == ugly => {
                            yet_3_5.extend(yet_2_3_5.pop_front());
                        }
                        Some(x) if x > ugly => { /* no-op for next use */ }
                        _ => unreachable!(),
                    }
                    yet_2_3_5.push_back(ugly);
                    (ugly, yet_2_3_5, yet_3_5, yet_5)
                },
            )
            .0
            .try_into()
            .expect("`usize` overflow `i32`")
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
}
