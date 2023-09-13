//! Given a string s, find the longest palindromic subsequence's length in s.
pub struct Solution;

macro_rules! translate_2d_to_idx {
    ($param: expr) => {
        |i: usize, j: usize| {
            (if 0 == i % 2 {
                i / 2 * (2 * $param - i + 1)
            } else {
                (2 * $param - i + 1) / 2 * i
            }) + j
        }
    };
}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        /*
         * Dynamic Programming
         *
         * For each interval [i..j], record its answer.
         *
         * Consider [i..(j+1)]
         *
         * Suppose [i..j] gives m and [(i+1)..(j+1)] gives n,
         * if we have a longer one with [i..(j+1)],
         * since that guy can't possibly be in [i..j],
         * meaning it contains [j],
         * since that guy can't possibly be in [(i+1)..(j+1)],
         * meaning it contains also [i],
         * hence [i] == [j].
         *
         * Conversely, suppose [i] == [j], consider what would it be like
         * to be the longest palindrome in [i..(j+1)]
         * WLOG suppose longest of [i..(j+1)] is contained in [i..j]
         * Case 1: it actually fits in [(i+1)..j]
         * then with addition of [i] and [j] attached to begin and end,
         * we get a longer one, contradiction.
         * Case 2: it doesn't fit in [(i+1)..j], then it contains [i]
         * meaning the last location is also a character equal to [i]
         * and since by premise we have [i] == [j], swap, and boom we get also
         * a longest palindrome using both [i] and [j].
         *
         * Hence for any [i..(j+1)],
         * [i] == [j] iff longest contains both [i] and [j] are in some longest.
         *
         * Thus, for [i..(j+1)],
         * if [i] == [j], then ask [(i+1)..j]
         * if [i] != [j], take the longer from [i..j] and [(i+1)..(j+1)]
         *
         * This could be done in triangular array (O(n^2) space) with naive
         * implementations, but it turns out we could fit them in linear
         * space since we don't really need to look back.
         */
        let s = s.chars().collect::<Vec<_>>();
        let get_2d = translate_2d_to_idx!(s.len());
        let mut dp_arr: Vec<usize> = Vec::with_capacity(get_2d(s.len() - 1, 0) + 1);
        (1..=s.len()).for_each(|substring_len| match substring_len {
            1 => s.iter().for_each(|_| dp_arr.push(1)),
            2 => s
                .iter()
                .zip(s.iter().skip(1))
                .for_each(|(c, d)| dp_arr.push(if c == d { 2 } else { 1 })),
            _ => {
                (0..s.len() - substring_len + 1).for_each(|i| {
                    let j = i + substring_len - 1;
                    dp_arr.push({
                        if s[i] == s[j] {
                            dp_arr.get(get_2d(substring_len - 3, i + 1)).unwrap() + 2
                        } else {
                            std::cmp::max(
                                dp_arr.get(get_2d(substring_len - 2, i)).unwrap().to_owned(),
                                dp_arr
                                    .get(get_2d(substring_len - 2, i + 1))
                                    .unwrap()
                                    .to_owned(),
                            )
                        }
                    });
                });
            }
        });
        dbg!(&dp_arr);
        dbg!(get_2d(s.len() - 1, 0));
        dp_arr.get(get_2d(s.len() - 1, 0)).unwrap().to_owned() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let s = "pppap".to_owned();
        // "pppp"
        assert_eq!(Solution::longest_palindrome_subseq(s), 4);
        let s = "cbba".to_owned();
        // "bb"
        assert_eq!(Solution::longest_palindrome_subseq(s), 2);
        let s = "aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa".to_owned();
        // "??????"
        assert_eq!(Solution::longest_palindrome_subseq(s), 0);
    }
}
