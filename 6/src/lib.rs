pub struct Solution;

/// Make zigzag version of the string.
/// ```
/// use lc_6_zigzag_conversion::Solution;
/// let s = "PAYPALISHIRING".to_string();
/// let num_rows = 3;
/// assert_eq!(Solution::convert(s, num_rows).as_str(), "PAHNAPLSIIGYIR");
/// let s = "PAYPALISHIRING".to_string();
/// let num_rows = 4;
/// assert_eq!(Solution::convert(s, num_rows).as_str(), "PINALSIGYAHRPI");
/// ```
/// Since in Zigzag we'd have...
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
/// And...
/// P     I     N
/// A   L S   I G
/// Y A   H R
/// P     I
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        use std::iter;
        let num_rows = num_rows as usize;
        let zigzag_offset = (num_rows - 1) * 2;
        if zigzag_offset == 0 {
            return s;
        }
        let mut ret = String::with_capacity(s.len());
        ret.push_str(&s.chars().step_by(zigzag_offset).collect::<String>());
        let mut scratch = String::new();
        for i in 1..num_rows - 1 {
            // Consider the following
            // P   A   H   N
            // A P L S I I G
            // Y   I   R
            // Then for the 2nd row, can decompose as following:
            // A   L   I   G shall be in pt0,
            //   P   S   I   shall be in pt1.
            //
            // Consider the following
            // P     I     N
            // A   L S   I G
            // Y A   H R
            // P     I
            // Then for the 2nd row, can decompose as following:
            // A     S     G shall be in pt0,
            //     L     I   shall be in pt1.
            //
            // Notice it's possible pt0 is longer;
            // Pad and use `Option` to yield if necessary.
            let new_row_pt0 = s.chars().skip(i).step_by(zigzag_offset);
            let new_row_pt1 = s
                .chars()
                .map(|ch| Some(ch))
                .skip(zigzag_offset - i)
                .step_by(zigzag_offset)
                .chain(iter::once(None));
            scratch.clear();
            new_row_pt0
                .zip(new_row_pt1)
                .for_each(|(ch_0, option_ch_1)| {
                    scratch.push(ch_0);
                    if let Some(ch_1) = option_ch_1 {
                        scratch.push(ch_1);
                    }
                });
            ret.push_str(&scratch);
        }
        ret.push_str(
            s.chars()
                .skip(num_rows - 1)
                .step_by(zigzag_offset)
                .collect::<String>()
                .as_str(),
        );
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::convert("f".to_string(), 1), "f".to_string());
    }
}
