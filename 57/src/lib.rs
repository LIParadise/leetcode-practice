struct Solution;

/*
 * Check if the closed interval contains num
 */
fn in_interval<T: PartialOrd + PartialEq>(num: &T, int: &[T]) -> bool {
    assert_eq!(int.len(), 2);
    assert_eq!(int[1] >= int[0], true);
    int[0] <= *num && *num <= int[1]
}

/*
 * Overlap example
 *     [  i0   ]
 *          [    i1     ]
 * Assume that intervals are written in [small, large]
 * Closed interval:
 * [2, 3] and [3, 4] are counted as overlap.
 *
 * True iff there's one of one's endpoint is in the other
 */
fn interval_overlap<T: PartialOrd + PartialEq>(i0: &[T], i1: &[T]) -> bool {
    assert_eq!(i0.len(), 2);
    assert_eq!(i1.len(), 2);
    assert_eq!(i0[1] >= i0[0], true);
    assert_eq!(i1[1] >= i1[0], true);
    in_interval(&i0[0], i1)
        || in_interval(&i0[1], i1)
        || in_interval(&i1[0], i0)
        || in_interval(&i1[1], i0)
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut merged_int = new_interval.clone();
        // Since no mut, can't use `retain`
        // Use `into_iter` instead
        let mut ret = intervals
            .into_iter()
            .filter(|interval| {
                let keep_this_interval = !interval_overlap(interval, &new_interval[..]);
                if !keep_this_interval {
                    // overlap
                    // update the merging interval
                    if interval[0] < merged_int[0] {
                        merged_int[0] = interval[0];
                    }
                    if interval[1] > merged_int[1] {
                        merged_int[1] = interval[1];
                    }
                }
                keep_this_interval
            })
            .collect::<Vec<_>>();
        let idx = match ret
            .iter()
            .enumerate()
            .find(|(_, interval)| interval[0] > merged_int[0])
        {
            Some(enum_iter) => enum_iter.0,
            None => ret.len(),
        };
        ret.insert(idx, merged_int);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_interval() {
        let num = 1;
        let int = vec![1, 3];
        assert_eq!(in_interval(&num, &int[..]), true);
    }
    #[test]
    fn test_interval_overlap() {
        assert_eq!(interval_overlap(&[0, 3][..], &[3, 5][..]), true);
        assert_eq!(interval_overlap(&[0, 2][..], &[3, 5][..]), false);
        assert_eq!(interval_overlap(&[3, 5][..], &[2, 5][..]), true);
        assert_eq!(interval_overlap(&[3, 5][..], &[1, 2][..]), false);
        assert_eq!(interval_overlap(&[3, 5][..], &[1, 3][..]), true);
        assert_eq!(interval_overlap(&[2, 8][..], &[3, 6][..]), true);
    }
    #[test]
    fn test_insert() {
        let ex_0_intervals = vec![vec![1, 3], vec![6, 9]];
        let ex_0_newinterval = vec![2, 5];
        let ex_0_ans = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(Solution::insert(ex_0_intervals, ex_0_newinterval), ex_0_ans);
        let ex_1_intervals = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]];
        let ex_1_newinterval = vec![4, 8];
        let ex_1_ans = vec![vec![1,2],vec![3,10],vec![12,16]];
        assert_eq!(Solution::insert(ex_1_intervals, ex_1_newinterval), ex_1_ans);
    }
}
