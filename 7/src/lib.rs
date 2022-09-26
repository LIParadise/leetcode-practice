pub trait LargeNum {
    type Output;
    const BASE: i32;
    fn into(self) -> Option<Self::Output>;
}

impl LargeNum for &str {
    type Output = i32;
    const BASE: i32 = 10;
    fn into(self) -> Option<Self::Output> {
        if Self::BASE <= 0 {
            return None;
        }
        let (pos, iter) = if '-' == self.chars().nth(0)? {
            (false, self.chars().skip(1))
        } else {
            (true, self.chars().skip(0))
        };
        iter.fold(Some(0), |num, ch| {
            let tmp = num.unwrap().checked_mul(Self::BASE)?;
            if pos {
                tmp.checked_add(ch.to_digit(Self::BASE as u32).map(|u| u as Self::Output)?)
            } else {
                tmp.checked_sub(ch.to_digit(Self::BASE as u32).map(|u| u as Self::Output)?)
            }
        })
    }
}

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let rev_x_preserve_sign: String = if x > 0 {
            x.to_string().chars().rev().collect()
        } else {
            x.to_string()
                .chars()
                .take(1)
                .chain(x.to_string().chars().filter(|d| d.is_numeric()).rev())
                .collect()
        };
        if let Some(ret) = LargeNum::into(rev_x_preserve_sign.as_str()) {
            ret
        }else{
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_largenum() {
        assert_eq!(Some(120), LargeNum::into("120"));
        assert_eq!(Some(i32::MAX), LargeNum::into("2147483647"));
        assert_eq!(Some(i32::MIN), LargeNum::into("-2147483648"));
        assert_eq!(Some(-1), LargeNum::into("-1"));
        assert_eq!(None, LargeNum::into("2147483648"));
        assert_eq!(None, LargeNum::into("9646324351"));
    }
    #[test]
    fn test_solution() {
        assert_eq!(21, Solution::reverse(120));
        assert_eq!(-123, Solution::reverse(-321));
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(0, Solution::reverse(1534236469));
    }
}
