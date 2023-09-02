pub struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let aliss_debt = alice_values
            .iter()
            .zip(bob_values.iter())
            .fold(0, |sum, (a, b)| sum + b - a);
        let mut consensus_worths = alice_values;
        consensus_worths
            .iter_mut()
            .zip(bob_values.iter())
            .for_each(|(n, b)| *n = *n + b);
        consensus_worths.sort_unstable();
        match consensus_worths
            .iter()
            .rev()
            .enumerate()
            .fold(0, |alice_margin_in_terms_of_consensus_worth, (idx, consensus)| {
                if idx % 2 == 0 {
                    alice_margin_in_terms_of_consensus_worth + consensus
                } else {
                    alice_margin_in_terms_of_consensus_worth - consensus
                }
            })
            .cmp(&aliss_debt)
        {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let alice_values = vec![1, 3];
        let bob_values = vec![2, 1];
        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), 1);
        let alice_values = vec![1, 2];
        let bob_values = vec![3, 1];
        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), 0);
        let alice_values = vec![2, 4, 3];
        let bob_values = vec![1, 6, 7];
        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), -1);
    }
}
