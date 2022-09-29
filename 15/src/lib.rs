pub struct Solution;

impl Solution {
    /// Find all unique triplets s.t. summation is 0.
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Sort the array first; supp. each x is a number
        //
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //
        // First, consider what triplets would be valid if we insist two of the
        // elements in triplet are exactly the 1st and the 2nd element.
        //
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^
        //
        // Since sorted, binary search
        // Range is in [2nd..]
        // If found viable choice, record it.
        //
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^               ... (Searching)
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^             ^     (Match! Record it.)
        //
        // Proceed to ask what if instead of [1st, 2nd, ?] we find s.t.
        // [1st, 3rd, ?] sum to 0.
        // Since sorted, start *from last match*, denoted by 'V', and
        // go *backwards*.
        //
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^     ...     V     (Searching)
        // Array: xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        //        ^^         ^   V     (Match! Record it.)
        //
        // Skip computation if the sum of the pair we choose
        // ([1st, 2nd], [1st, 3rd], etc) have positive sum.

        let mut nums = nums;
        nums.sort_unstable();
        let nums = nums;
        // Fixed memory for entire loop
        // The triplet is [i, j, k] s.t. i <= j <= k
        let mut triplet = Vec::from([0; 3]);

        let mut ret =
            nums.iter()
                .enumerate()
                .fold(Vec::new(), |mut ret: Vec<Vec<i32>>, (idx, i)| {
                    // idx_for_i, idx_for_j, idx_for_k => idx, jdx, kdx
                    if i <= &0 {
                        let kdx = nums.len();
                        // i <= j <= k; possible triplet only if `i` non-positive
                        nums.iter().enumerate().skip(idx + 1).for_each(|(jdx, j)| {
                            // i <= j <= k; possible triplet only if `i+j` non-positive
                            if i + j <= 0 {
                                let kdx = nums.as_slice()[jdx + 1..kdx]
                                    .partition_point(|kandidate| i + j + kandidate < 0)
                                    + (jdx + 1);
                                match nums.get(kdx) {
                                    Some(k) if i + j + k == 0 => {
                                        triplet[0] = i.clone();
                                        triplet[1] = j.clone();
                                        triplet[2] = k.clone();
                                        ret.push(triplet.clone());
                                    }
                                    _ => {}
                                }
                            }
                        });
                    }
                    ret
                });

        // Uniqueness of triplet
        ret.sort_unstable_by(|triplet_i, triplet_j| {
            // dictionary order
            // compare only first two is enough since sum is 0
            if triplet_i[0] < triplet_j[0] {
                std::cmp::Ordering::Less
            } else if triplet_i[0] == triplet_j[0] && triplet_i[1] < triplet_j[1] {
                std::cmp::Ordering::Less
            } else if triplet_i[0] == triplet_j[0] && triplet_i[1] == triplet_j[1] {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            }
        });
        ret.dedup();
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_soln() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        // Notice this above have only ONE [-1, 0, 1],
        // even when there's two -1 in input.
        assert_eq!(output, Solution::three_sum(input));
        let input = vec![0, 1, 1];
        let output: Vec<Vec<i32>> = Vec::new();
        assert_eq!(output, Solution::three_sum(input));
        let input = vec![0, 0, 0];
        let output = vec![vec![0, 0, 0]];
        assert_eq!(output, Solution::three_sum(input));
        let input = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
        let output = vec![
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-3, -1, 4],
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, -1, 2],
            vec![-1, 0, 1],
        ];
        assert_eq!(output, Solution::three_sum(input));
    }
}
