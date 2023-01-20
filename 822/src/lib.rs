pub struct Solution;

impl Solution {
    /// You are given two 0-indexed integer arrays fronts and backs of length n,
    /// where the ith card has the positive integer fronts[i] printed on the
    /// front and backs[i] printed on the back. Initially, each card is placed
    /// on a table such that the front number is facing up and the other is
    /// facing down.
    /// You may flip over any number of cards (possibly zero).
    ///
    /// After flipping the cards, an integer is considered good if it is facing
    /// down on some card and not facing up on any card.
    ///
    /// Return the minimum possible good integer after flipping the cards.
    /// If there are no good integers, return 0.
    // A leetcode's own tricky way of describing finding XOR of two sets
    // and its minimum element.
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        use std::collections::BTreeSet as bts;
        let mut candidate = bts::new();
        let mut exclude = bts::new();
        fronts.iter().zip(backs.iter()).for_each(|(f, b)| {
            if f != b {
                candidate.insert(f);
                candidate.insert(b);
            } else {
                exclude.insert(f);
            }
        });
        match candidate.difference(&exclude).next() {
            Some(ans) => **ans,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
