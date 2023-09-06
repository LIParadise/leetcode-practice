pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let end_of_last = s.char_indices().rev().try_for_each(|(idx, ch)| {
            // ch.is_whitespace().then_some(()).ok_or(idx)
            // Leetcode is too lazy to update its Rust compiler
            // so here's the modified version
            if ch.is_whitespace() {
                Ok(())
            } else {
                Err(idx)
            }
        });
        match end_of_last {
            Ok(()) => {
                // no char boundary found, it's basically bunch of spaces
                0
            }
            Err(back_of_last) => {
                let before_begin_of_last = s
                    .char_indices()
                    .rev()
                    .skip(s.len() - back_of_last)
                    .try_for_each(|(idx, ch)| {
                        {
                            // !ch.is_whitespace()).then_some(()).ok_or(idx)
                            // Leetcode is too lazy to update its Rust compiler
                            // so here's the modified version
                            if ch.is_whitespace() {
                                Err(idx)
                            } else {
                                Ok(())
                            }
                        }
                    })
                    .err();

                (if let None = before_begin_of_last {
                    back_of_last + 1
                } else {
                    back_of_last - before_begin_of_last.unwrap()
                }) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let s = String::from(" hello ");
        assert_eq!(Solution::length_of_last_word(s), 5);
        let s = String::from(" hello , world heh      ");
        assert_eq!(Solution::length_of_last_word(s), 3);
        let s = String::from("a");
        assert_eq!(Solution::length_of_last_word(s), 1);
    }
}
