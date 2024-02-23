pub struct Solution;

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        // Since all integer, division makes it no larger.
        // The answer is quite straightforward.
        //
        // In general, this problem takes O(n^3), via DP.
        let mut ret = Vec::new();
        ret.extend(
            nums.iter()
                .enumerate()
                .map(|(i, n)| {
                    let mut tmp = Vec::with_capacity(10);
                    if i == 1 && nums.len() > 2 {
                        tmp.push('(' as u8);
                    }
                    tmp.extend(n.to_string().as_bytes());
                    if i != nums.len() - 1 {
                        tmp.push('/' as u8);
                    } else if nums.len() > 2 {
                        tmp.push(')' as u8);
                    }
                    tmp
                })
                .flatten(),
        );
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)".to_owned()
        );
        assert_eq!(
            Solution::optimal_division(vec![2, 3, 4]),
            "2/(3/4)".to_owned()
        );
    }
}
