pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut right = chars.len() - 1;
        while right > 0 {
            let left = right
                - chars
                    .iter()
                    .take(right + 1)
                    .rev()
                    .zip(chars.iter().take(right + 1).rev().skip(1))
                    .take_while(|(a, b)| a == b)
                    .count();
            if left < right {
                chars.as_mut_slice()[left + 1..=right]
                    .iter_mut()
                    .zip(
                        (right - left + 1)
                            .to_string()
                            .as_str()
                            .chars()
                            .chain((0..).map(|_| 0 as char)),
                    )
                    .for_each(|(ret, c)| {
                        *ret = c;
                    })
            }
            right = left.saturating_sub(1);
        }
        while let Some(idx) = chars.iter().rev().position(|&c| c == 0 as char) {
            let idx = chars.len() - 1 - idx;
            chars.remove(idx);
        }
        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let mut s = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        Solution::compress(&mut s);
        assert_eq!(s, vec!['a', '2', 'b', '2', 'c', '3']);
        let mut s = vec!['a'];
        Solution::compress(&mut s);
        assert_eq!(s, vec!['a']);
        let mut s = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        Solution::compress(&mut s);
        assert_eq!(s, vec!['a', 'b', '1', '2']);
    }
}
