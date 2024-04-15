pub struct Solution;

impl Solution {
    /// Consider array of length 2 with partial order defined as following:
    /// a > b iff (a[0] > b[0]) && (a[1] > b[1])
    ///
    /// Given array of arrays of length 2,
    /// return # of elements that are not maximal
    // O(n^2) brute force not possible since input length spec goes up to 10^5
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        // Idea:
        //
        // Sort the array by first member then second member, i.e.
        //
        // [[x_0, x_0_0], [x_0, x_0_1], [x_0, x_0_2], ..., [x_0, x_0_{m_0}],
        //  [x_1, x_1_0], [x_1, x_1_1], [x_1, x_1_2], ..., [x_1, x_1_{m_1}],
        //  [x_2, x_2_0], [x_2, x_2_1], [x_2, x_2_2], ..., [x_2, x_2_{m_2}],
        //  ...
        //  [x_n, x_n_0], [x_n, x_n_1], [x_n, x_n_2], ..., [x_n, x_n_{m_n}]]
        //
        // In this view, consider those elements with first entry x_i
        // (there are m_i such elements)
        // How many of them are not maximal?
        //
        // Let M = max of [x_j_{m_j}] for j in (i+1)..=n
        // i.e. the max by second entriy in each "row-wise view"
        // i.e. the maxes of which share the same first element
        // Then the answer is how many x_i_j is less than M, j in 0..=(m_i)
        //
        // In layman's terms, for example, [x_0, x_0_0] is considered as
        // not maximal iff there's some i s.t.
        // x_0 < x_i and x_0_0 < x_i_{m_i}
        //
        //
        //
        // Time Complexity:
        //
        // O(n*lg(n)), since sort then some linear scans
        //
        // Space Compexity:
        //
        // Linear, since auxiliary storage may be as large as input
        // e.g. when all elements have different first entry
        if properties.len() <= 1 {
            return 0;
        }
        properties.sort_unstable();
        #[cfg(test)]
        {
            println!("prop{:?}", properties);
        }
        let first_col_of_rows = std::iter::once(0)
            .chain(
                properties
                    .iter()
                    .zip(properties.iter().enumerate().skip(1))
                    .filter_map(|(a, (i, b))| if a[0] != b[0] { Some(i) } else { None }),
            )
            .collect::<Vec<usize>>();
        #[cfg(test)]
        {
            println!("fcor{:?}", first_col_of_rows);
        }
        let last_col_of_rows = first_col_of_rows
            .iter()
            .skip(1)
            .map(|i| i - 1)
            .chain(std::iter::once(properties.len() - 1))
            .map(|i| &properties[i][1])
            .collect::<Vec<_>>();
        #[cfg(test)]
        {
            println!("lcor{:?}", last_col_of_rows);
        }
        let maximals = last_col_of_rows
            .iter()
            .rev()
            .scan(None, |m, &x| {
                match m {
                    None => *m = Some(x),
                    Some(m) => *m = std::cmp::max(x, *m),
                };
                Some(*m)
            })
            .flatten()
            .collect::<Vec<_>>();
        #[cfg(test)]
        {
            println!("maximals{:?}", maximals);
        }
        first_col_of_rows
            .iter()
            .zip(first_col_of_rows.iter().skip(1))
            .zip(maximals.iter().rev().skip(1))
            .fold(0, |ret, ((&start, &end), &&maximal)| {
                if let Some(cnt) = &properties[start..end].iter().position(|n| n[1] >= maximal) {
                    ret + cnt
                } else {
                    end - start + ret
                }
            })
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
            0
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![3, 3], vec![2, 2]]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![
                vec![1, 1],
                vec![2, 1],
                vec![2, 2],
                vec![1, 2]
            ]),
            1
        );
    }
}
