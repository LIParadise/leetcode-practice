struct MyI32(i32);
impl From<&str> for MyI32 {
    fn from(s: &str) -> Self {
        const BASE_U: u32 = 10;
        const BASE_I: i32 = 10;
        use i32 as OutputType;
        let (pos, mut iter) = if let Some('-') = s.chars().next() {
            (false, s.chars().skip(1))
        } else {
            (true, s.chars().skip(0))
        };

        match iter.try_fold(0, |num: i32, ch| {
            let tmp = num.checked_mul(BASE_I)?;
            if pos {
                tmp.checked_add(ch.to_digit(BASE_U).map(|u| u as OutputType)?)
            } else {
                tmp.checked_sub(ch.to_digit(BASE_U).map(|u| u as OutputType)?)
            }
        }) {
            Some(i) => i.into(),
            None => 0.into(),
        }
    }
}
impl From<MyI32> for i32 {
    fn from(i: MyI32) -> Self {
        i.0
    }
}
impl From<i32> for MyI32 {
    fn from(i: i32) -> Self {
        MyI32(i)
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
        let ret: MyI32 = rev_x_preserve_sign.as_str().into();
        ret.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_solution() {
        assert_eq!(21, Solution::reverse(120));
        assert_eq!(-123, Solution::reverse(-321));
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(0, Solution::reverse(1534236469));
    }
}
