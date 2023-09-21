/// Alice and Bob take turns playing a game, with Alice starting first.
/// There are n stones arranged in a row. On each player's turn, while the
/// number of stones is more than one, they will do the following:
///
/// 1. Choose an integer x > 1, and remove the leftmost x stones from the row.
/// 2. Add the sum of the removed stones' values to the player's score.
/// 3. Place a new stone, whose value is equal to that sum, on the left side of the row.
///
/// The game stops when only one stone is left in the row.
/// The score difference between Alice and Bob is
/// (Alice's score - Bob's score).
/// Alice's goal is to maximize the score difference,
/// and Bob's goal is the minimize the score difference.
///
/// Given an integer array stones of length n where stones[i] represents the
/// value of the ith stone from the left, return the score difference between
/// Alice and Bob if they both play optimally.
// Hint: Since the value put back is both sum and score, the index is the only
// thing that matters.
pub struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        // Suppose we have {a_1, a_2, ..., a_n}
        // Let partial sum be {s_1, s_2, ..., s_n}
        // Notice each step we're facing with exactly the same type
        // of subproblem; it's just the numbers are now
        // {s_k, a_{k+1}, a_{k+2}, ..., a_n}
        // Why the same?
        // It's since both player's strategy are the same: maximize their
        // advantage over the other.
        //
        // Consider for example [-5, 7, -3, 2, -3]
        // partial sum array    [-5, 2, -1, 1, -2]
        // Subproblem 1:                      [-2]
        // Subproblem 2:                   [1, -3]
        // Subproblem 3:               [-1, 2, -3]
        // Subproblem 4:            [2, -3, 2, -3]
        // Subproblem 5:        [-5, 7, -3, 2, -3]
        // We record for each subproblem what the winner difference would be,
        // [-2]: 0 (No choice)
        // [1, -3]: -2 (Only 1 choice)
        // [-1, 2, -3]:
        // Either we take (-1+2) = 1 , then the other guy takes -2, diff = 3
        // or we take all (-1+2-3) = -2, then the other guy takes 0, diff = -2
        // Why? we know exactly what the other guy would take!
        // Hence we take (-1+2) = 1 to maximize our difference.
        // Notice what numbers we're considering:
        // THEY'RE IN THE FORM OF (s_k - (answer to subproblem #k))
        //
        // Carrying out the calculations and align the results, we have:
        // Original Array:   [-5, 7, -3, 2, -3]
        // Partial Sum Array [-5, 2, -1, 1, -2]
        // Winner Differnce    3  3   3 -2   0
        //
        // For the partial sums, leetcode specifies that input is s.t. must
        // be covered by `i32`.
        let partial_sums = stones
            .iter()
            .scan(0, |sum, stone| {
                Some({
                    *sum += stone;
                    sum.clone()
                })
            })
            .collect::<Vec<_>>();
        partial_sums
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .fold(partial_sums.last().unwrap().clone(), |diff, partial_sum| {
                std::cmp::max(diff, partial_sum - diff)
            })
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let input_arr = vec![-1, 2, -3, 4, -5];
        assert_eq!(Solution::stone_game_viii(input_arr), 5);
        let input_arr = vec![7, -6, 5, 10, 5, -2, -6];
        assert_eq!(Solution::stone_game_viii(input_arr), 13);
        let input_arr = vec![-10, -12];
        assert_eq!(Solution::stone_game_viii(input_arr), -22);
    }
}
