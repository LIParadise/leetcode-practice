macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}

use self::rational::Rational;

pub struct Solution;

mod rational {
    use std::ops::{Add, Mul, Sub};
    pub fn euclid(mut a: u32, mut b: u32) -> Option<u32> {
        match (a == 0, b == 0) {
            (true, true) => None,
            (true, false) => Some(b),
            (false, true) => Some(a),
            (false, false) => Some(loop {
                let r = a % b;
                if r == 0 {
                    break b;
                } else {
                    a = b;
                    b = r;
                }
            }),
        }
    }

    #[derive(Clone, Copy, Debug)]
    /// A simple rational number implementation: sign, numerator, denominator
    /// note that 0 is always positive and it's always reduced
    pub struct Rational((bool, u32, u32));

    impl Default for Rational {
        fn default() -> Self {
            Self((true, u32::default(), 1))
        }
    }
    impl Rational {
        // only positive 0 is allowed
        pub fn new(positive: bool, numerator: u32, denominator: u32) -> Option<Self> {
            if denominator == 0 {
                None
            } else {
                let positive = if numerator == 0 { true } else { positive };
                let factor = euclid(numerator, denominator).unwrap();
                let numerator = numerator / factor;
                let denominator = denominator / factor;
                Some(Self((positive, numerator, denominator)))
            }
        }
    }

    impl Add for Rational {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let me = self.as_ref();
            let rhs = rhs.as_ref();
            // denominators should never be zero, checks skipped
            let factor = euclid(me.2, rhs.2).unwrap();
            let denominator = me.2 / factor * rhs.2;
            let me_numerator = me.1 * (rhs.2 / factor);
            let rhs_numerator = rhs.1 * (me.2 / factor);
            if me.0 == rhs.0 {
                let numerator = me_numerator + rhs_numerator;
                let factor = euclid(denominator, numerator).unwrap();
                Self::new(me.0, numerator / factor, denominator / factor).unwrap()
            } else {
                // shall not produce "negative 0"
                let positive =
                    me.0 && me_numerator >= rhs_numerator || rhs.0 && rhs_numerator >= me_numerator;
                let numerator = me_numerator.abs_diff(rhs_numerator);
                let factor = euclid(denominator, numerator).unwrap();
                Self::new(positive, numerator / factor, denominator / factor).unwrap()
            }
        }
    }
    impl Mul for Rational {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            // denominators should never be zero, checks skipped
            let me = self.as_ref();
            let rhs = rhs.as_ref();
            let factor = euclid(me.2, rhs.2).unwrap();
            let denominator = me.2 / factor * rhs.2;
            let numerator = me.1 * rhs.1 / factor;
            let positive = if numerator == 0 { true } else { me.0 ^ rhs.0 };
            Self((positive, numerator, denominator))
        }
    }
    impl Mul for &Rational {
        type Output = Rational;
        fn mul(self, rhs: Self) -> Self::Output {
            *self + *rhs
        }
    }
    impl Add for &Rational {
        type Output = Rational;
        fn add(self, rhs: Self) -> Self::Output {
            *self + *rhs
        }
    }

    impl PartialOrd for Rational {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let self_minus_other = self + &(other * &Self((false, 1, 1)));
            let self_minus_other = self_minus_other.as_ref();
            if self_minus_other.0 && self_minus_other.1 != 0 {
                Some(std::cmp::Ordering::Greater)
            } else if self_minus_other.1 == 0 {
                Some(std::cmp::Ordering::Equal)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        }
    }
    impl PartialEq for Rational {
        fn eq(&self, other: &Self) -> bool {
            self <= other && other <= self
        }
    }

    impl Sub for Rational {
        type Output = Rational;
        fn sub(self, rhs: Self) -> Self::Output {
            if rhs == Self::default() {
                self
            } else {
                self + rhs * Self::new(false, 1, 1).unwrap()
            }
        }
    }
    impl Sub for &Rational {
        type Output = Rational;
        fn sub(self, rhs: Self) -> Self::Output {
            *self - *rhs
        }
    }

    impl AsRef<(bool, u32, u32)> for Rational {
        fn as_ref(&self) -> &(bool, u32, u32) {
            &self.0
        }
    }

    impl From<Rational> for String {
        fn from(value: Rational) -> Self {
            let num = value.as_ref();
            format!("{}{}/{}", if num.0 { "" } else { "-" }, num.1, num.2)
        }
    }
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut expression: &str = expression.as_ref();
        let mut ret = Rational::default();
        loop {
            let token_end = expression
                .chars()
                // token is like the following regular expression:
                // '[-+]?[[:digit:]]+\/[[:digit:]]+'
                // note that we may or may not have sign,
                // skip 1 is ok since there's at least one digit, slash, and yet another digit
                .enumerate()
                .position(|(idx, ch)| idx != 0 && (ch == '+' || ch == '-'))
                .unwrap_or(expression.len());
            let token = &expression[..token_end];
            let token = Self::parse_token(token);
            lprintln!("{}, {}, {:?}", expression, token_end, token);
            if let Some(token) = token {
                ret = ret + token;
            } else {
                break;
            }
            expression = &expression[token_end..];
        }
        ret.into()
    }
    fn parse_token(s: &str) -> Option<rational::Rational> {
        lprintln!("parsing {}", s);
        s.chars().position(|ch| ch == '/').and_then(|division_idx| {
            let positive;
            let denominator = s[division_idx + 1..].parse::<u32>().ok();
            let numerator = match &s[..division_idx] {
                numerator if numerator.starts_with('-') => {
                    positive = false;
                    numerator[1..].parse::<u32>().ok()
                }
                numerator => {
                    positive = true;
                    numerator.parse::<u32>().ok()
                }
            };
            lprintln!(
                "assemble {:?}, {:?}, {:?}",
                positive,
                numerator,
                denominator
            );
            denominator.and_then(|denominator| {
                numerator
                    .and_then(|numerator| rational::Rational::new(positive, numerator, denominator))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{rational::*, Solution};
    #[test]
    fn test_utils() {
        let r0 = Rational::new(false, 0, 1);
        let r1 = Rational::new(true, 0, 1);
        assert_eq!(r0, r1);
        let r0 = Rational::new(false, 0, 1).unwrap();
        let r1 = Rational::new(true, 0, 1).unwrap();
        assert_eq!(r0 * r1 * r1, Rational::default());
        let r0 = Rational::new(false, 2, 3).unwrap();
        let r1 = Rational::new(true, 2, 7).unwrap();
        assert_eq!(r0 * r1 * r1, Rational::new(false, 8, 147).unwrap());
        assert!(r0 < r1);
        assert!(r0 * r1 < r0 * r1 * r1);
        assert_eq!(
            r0 * r1 - r0 * r1 * r1,
            Rational::new(false, 20, 147).unwrap()
        );
        let r0 = Rational::new(false, 2, 4).unwrap();
        let r1 = Rational::new(true, 2, 8).unwrap();
        assert_eq!(r0 * r1 * r1, Rational::new(false, 2, 64).unwrap());
        assert!(r0 < r1);
        assert!(r0 * r1 < r0 * r1 * r1);
        assert_eq!(r0 * r1 - r0 * r1 * r1, Rational::new(false, 9, 96).unwrap());
    }

    #[test]
    fn test_soln() {
        let expression = "-1/2+1/2".to_owned();
        let output = "0/1".to_owned();
        assert_eq!(Solution::fraction_addition(expression), output);
        let expression = "-1/2+1/2+1/3".to_owned();
        let output = "1/3".to_owned();
        assert_eq!(Solution::fraction_addition(expression), output);
        let expression = "1/3-1/2".to_owned();
        let output = "-1/6".to_owned();
        assert_eq!(Solution::fraction_addition(expression), output);
    }
}
