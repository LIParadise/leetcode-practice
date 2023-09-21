pub struct Solution;

/// Given an array of strings nums containing n unique binary strings each
/// of length n, return a binary string of length n that does not appear
/// in nums.
///
/// If there are multiple answers, you may return any of them.
///
/// Elements in input are distinct and not exhaustive i.e. there must be
/// a string that's not included.
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| a.cmp(&b));
        match nums.first() {
            None => return "".to_owned(),
            Some(num) => {
                if 0 != usize::from_str_radix(&num, 2).unwrap() {
                    return String::from_utf8(vec!['0' as u8; num.len()]).unwrap();
                }
            }
        }
        nums.iter()
            .zip(nums.iter().skip(1))
            .try_for_each(|(s0, s1)| {
                let input_undefined_msg: &'static str = "Input should be binary string.";
                let n0 = usize::from_str_radix(&s0, 2).expect(input_undefined_msg);
                let n1 = usize::from_str_radix(&s1, 2).expect(input_undefined_msg);
                if n0 + 1 != n1 {
                    Err(Self::usize_to_string(n0 + 1, s0.len()))
                } else {
                    Ok(())
                }
            })
            .map_or_else(
                |ret| ret,
                |_| {
                    let l = nums.first().unwrap().len();
                    let mut ret = String::with_capacity(l);
                    (0..l).for_each(|_| ret.push('1'));
                    ret
                },
            )
    }
    fn usize_to_string(mut u: usize, n: usize) -> String {
        let mut ret = Vec::with_capacity(n);
        (0..n).for_each(|_| {
            ret.push(if u % 2 == 1 { '1' as u8 } else { '0' as u8 });
            u = u / 2;
        });
        ret.reverse();
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let strings = vec!["00", "01"];
        let strings: Vec<String> = strings.iter().map(|&s| s.to_owned()).collect();
        let copy = strings.clone();
        let ans = Solution::find_different_binary_string(strings);
        let strings = copy
            .iter()
            .map(|s| s.as_str())
            .collect::<std::collections::HashSet<_>>();
        assert!(!strings.contains(&ans[..]));

        let strings = vec!["01", "10"];
        let strings: Vec<String> = strings.iter().map(|&s| s.to_owned()).collect();
        let copy = strings.clone();
        let ans = Solution::find_different_binary_string(strings);
        let strings = copy
            .iter()
            .map(|s| s.as_str())
            .collect::<std::collections::HashSet<_>>();
        assert!(!strings.contains(&ans[..]));

        let strings = vec!["111", "011", "001"];
        let strings: Vec<String> = strings.iter().map(|&s| s.to_owned()).collect();
        let copy = strings.clone();
        let ans = Solution::find_different_binary_string(strings);
        let strings = copy
            .iter()
            .map(|s| s.as_str())
            .collect::<std::collections::HashSet<_>>();
        assert!(!strings.contains(&ans[..]));
    }
}
