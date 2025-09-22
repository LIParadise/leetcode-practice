pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // Two properties worth note:
        // 1. The problem is inherently symmetrical,
        //    in the sense that if string A takes at leas n edits to B,
        //    then B takes also at least n edits to A.
        //    Proof: every step is reversible.
        // 2. Say L no shorter than S,
        //    then optimal edit distance is no larger than length of L
        //    Proof: one by one replace L's char by that of S, delete tail.
        //    Example: (aaaaaaabbc, bbcdddd)
        if word1.is_empty() {
            return word2.len().try_into().unwrap();
        } else if word2.is_empty() {
            return word1.len().try_into().unwrap();
        } else {
            return Self::min_distance_worker(word1.as_bytes(), word2.as_bytes());
        }
    }
    // For char indices i in the longer and j in the shorter,
    // (there are square of them)
    // consider these subproblems:
    // 1. optimal edit is s.t. longer[i] replaced with shorter[j],
    //    record the cost (they may equal!) then continue with longer[i+1] and shorter[j+1]
    //    - They may equal, in which case this local cost is 0
    //    - Or they differ, in which case this local cost is 1
    // 2. optimal edit is s.t. longer[i] should be removed,
    //    record the cost (1) then continue with longer[i+1] and shorter[j]
    // 3. In both cases, either exhausted, then remaining cost is len of the remaining one
    /// Both shall be non-empty
    fn min_distance_worker(row_str: &[u8], col_str: &[u8]) -> i32 {
        // ┌───╥───┬───┬───┬───┬───┬───┐
        // │   ║ c │ o │ l │ s │ t │ r │ For X, it's subproblem "lstr" vs "wstr"
        // ╞═══╬═══╪═══╪═══╪═══╪═══╪═══╡ consider either to skip (delete) row -> "lstr" vs "str" -> a + 1
        // │ r ║   │   │   │   │   │   │                    skip (delete) col -> "str" vs "wstr" -> c + 1
        // ├───╫───┼───┼───┼───┼───┼───┤ or to morph -> "str" vs "str" -> b + (morph cost)
        // │ o ║   │   │   │   │   │   │ (which is 1 here since 'w' != 'l')
        // ├───╫───┼───┼───┼───┼───┼───┤
        // │ w ║   │   │ X │ c │   │   │
        // ├───╫───┼───┼───┼───┼───┼───┤ For base case Y, consider subproblem char 'r' vs substr "str",
        // │ s ║   │   │ a │ b │   │   │ in which case the answer is immediate:
        // ├───╫───┼───┼───┼───┼───┼───┤ one side is a char, answer is just len of substr,
        // │ t ║   │   │   │   │   │   │ except if substr contains the char, subtract 1, so 2 in this case.
        // ├───╫───┼───┼───┼───┼───┼───┤
        // │ r ║   │   │   │ Y │   │   │ Note we actually need only linear storage if go backwards!
        // └───╨───┴───┴───┴───┴───┴───┘
        let mut dp = vec![vec![0_u16; col_str.len()]; row_str.len()];
        {
            // base case: last row, one char vs substr
            let r = row_str.last().unwrap();
            dp.last_mut()
                .into_iter()
                .map(Vec::as_mut_slice)
                .flat_map(<[_]>::iter_mut)
                .rev()
                .zip(col_str.iter().rev().enumerate().scan(
                    false,
                    |substr_matched, (rev_idx, c)| {
                        if c == r {
                            *substr_matched = true;
                        }
                        Some((*substr_matched, rev_idx + 1))
                    },
                ))
                .for_each(|(elem, (substr_matched, len))| {
                    *elem = u16::try_from(len).unwrap() - if substr_matched { 1 } else { 0 };
                });

            // base case: last col, one char vs substr
            let c = col_str.last().unwrap();
            dp.iter_mut()
                .map(Vec::as_mut_slice)
                // `flat_map` is fine since rectangular
                .flat_map(<[_]>::last_mut)
                .rev()
                .zip(row_str.iter().rev().enumerate().scan(
                    false,
                    |substr_matched, (rev_idx, r)| {
                        if c == r {
                            *substr_matched = true;
                        }
                        Some((*substr_matched, rev_idx + 1))
                    },
                ))
                .for_each(|(elem, (substr_matched, len))| {
                    *elem = u16::try_from(len).unwrap() - if substr_matched { 1 } else { 0 };
                });
        }
        for r in (0..row_str.len()).rev().skip(1) {
            for c in (0..col_str.len()).rev().skip(1) {
                let _solving_subproblem = (&row_str[r..], &col_str[c..]);

                let cost_morph = {
                    // suppose `longer[l]` shall become `shorter[s]`...
                    let morph_char_cost = if row_str[r] == col_str[c] { 0 } else { 1 };
                    // consider subproblem `longer[l+1..]` vs `shorter[s+1..]`
                    // boundary is ok since we `Iterator::skip`
                    let remained_cost = dp[r + 1][c + 1];
                    remained_cost + morph_char_cost
                };
                let cost_delete_char_from_row = dp[r + 1][c] + 1;
                let cost_delete_char_from_col = dp[r][c + 1] + 1;
                dp[r][c] = std::cmp::min(
                    cost_morph,
                    std::cmp::min(cost_delete_char_from_col, cost_delete_char_from_row),
                )
                .try_into()
                .unwrap()
            }
        }
        i32::try_from(dp[0][0]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        test_helper("horse", "ros", 3);
        test_helper("intention", "execution", 5);
        test_helper("prosperity", "properties", 4);
        //              pro perity 1
        //              pro peries 2
        //              pro perties 1 => 4!!
        test_helper("sperity", "perties", 4);
        //            perity 1
        //            peries 2
        //            perties 1
    }

    fn test_helper(s1: &str, s2: &str, i: i32) {
        {
            let s1 = String::from(s1);
            let s2 = String::from(s2);
            assert_eq!(Solution::min_distance(s1, s2), i);
        }
        {
            let s1 = String::from(s1);
            let s2 = String::from(s2);
            assert_eq!(Solution::min_distance(s2, s1), i);
        }
    }
}
