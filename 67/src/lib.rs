pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = if a.len() >= b.len() {
            (a.as_bytes(), b.as_bytes())
        } else {
            (b.as_bytes(), a.as_bytes())
        };
        let mut ret = Vec::new();
        let mut carry = 0;
        a.iter()
            .rev()
            .zip(b.iter().rev().chain(std::iter::repeat(&('0' as u8))))
            .for_each(|(&a, &b)| {
                let a = a - '0' as u8;
                let b = b - '0' as u8;
                match carry + a + b {
                    3 => {
                        carry = 1;
                        ret.push(true)
                    }
                    2 => {
                        carry = 1;
                        ret.push(false)
                    }
                    1 => {
                        carry = 0;
                        ret.push(true)
                    }
                    0 => {
                        carry = 0;
                        ret.push(false)
                    }
                    _ => panic!("Addition fails: bit overflow"),
                }
            });
        if carry == 1 {
            ret.push(true);
        }
        ret.into_iter()
            .rev()
            .map(|b| b.then(|| '1').unwrap_or('0'))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            String::from("100")
        );
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            String::from("10101")
        );
    }
}
