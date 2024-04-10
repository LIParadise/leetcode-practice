pub struct Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        Self::winner_of_game_workder(&colors)
    }
    fn winner_of_game_workder(colors: &str) -> bool {
        #[derive(Debug, Clone, Copy)]
        enum State {
            FirstA,
            SecondA,
            FirstB,
            SecondB,
            Null,
        }
        let mut s = State::Null;
        let mut a_cnt = 0;
        let mut b_cnt = 0;
        colors.chars().for_each(|c| match (c, s) {
            ('A', State::SecondA) => a_cnt += 1,
            ('B', State::SecondB) => b_cnt += 1,
            ('A', State::SecondB) | ('A', State::FirstB) | ('A', State::Null) => s = State::FirstA,
            ('B', State::SecondA) | ('B', State::FirstA) | ('B', State::Null) => s = State::FirstB,
            ('A', State::FirstA) => s = State::SecondA,
            ('B', State::FirstB) => s = State::SecondB,
            _ => panic!("String not comprised only of 'A' and 'B', abort."),
        });
        return a_cnt > b_cnt;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::winner_of_game("ABBBBBBBAAA".to_owned()), false);
        assert_eq!(Solution::winner_of_game("AAABABB".to_owned()), true);
        assert_eq!(Solution::winner_of_game("AA".to_owned()), false);
    }
}
