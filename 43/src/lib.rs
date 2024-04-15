pub struct Solution;

static ERR_DIGIT_TOO_LARGE: &str = "Individual digit too large, abort.";

trait MyAddMul<T: std::iter::FromIterator<char>> {
    fn add(&self, other: &Self) -> T
    where
        Self: AsRef<str>,
    {
        let mut carry = 0_u8;
        let (a, b) = if self.as_ref().len() >= other.as_ref().len() {
            (self.as_ref(), other.as_ref())
        } else {
            (other.as_ref(), self.as_ref())
        };
        let mut ret = Vec::with_capacity(a.len());
        a.chars()
            .rev()
            .zip(b.chars().rev().chain(std::iter::repeat('0')))
            .for_each(|(a, b)| {
                let sum = TryInto::<u8>::try_into(a.to_digit(10).unwrap()).unwrap()
                    + TryInto::<u8>::try_into(b.to_digit(10).unwrap()).unwrap()
                    + carry;
                carry = match sum {
                    0..=9 => 0,
                    10..=19 => 1,
                    20..=27 => 2,
                    _ => panic!("{}", ERR_DIGIT_TOO_LARGE),
                };
                ret.push(sum % 10);
            });
        if carry != 0 {
            ret.push(carry);
        }
        T::from_iter(ret.iter().map(|u| (u + b'0') as char).rev())
    }
    fn shift_by_decimal(&self, u: usize) -> T
    where
        Self: AsRef<str>,
    {
        let mut ret = Vec::from_iter(self.as_ref().chars());
        (0..u).for_each(|_| {
            ret.push('0');
        });
        T::from_iter(ret)
    }
    fn mul_u(&self, u: u8) -> T
    where
        Self: AsRef<str>,
    {
        let mut ret = Vec::from_iter(
            self.as_ref()
                .chars()
                .map(|c| TryInto::<u8>::try_into(c.to_digit(10).unwrap()).unwrap()),
        );
        let mut carry = 0_u8;
        match u {
            0 => T::from_iter(std::iter::empty()),
            1 => T::from_iter(self.as_ref().chars()),
            2..=9 => {
                ret.iter_mut().rev().for_each(|digit| {
                    *digit = *digit * u + carry;
                    carry = match *digit {
                        0..=9 => 0,
                        10..=19 => 1,
                        20..=29 => 2,
                        30..=39 => 3,
                        40..=49 => 4,
                        50..=59 => 5,
                        60..=69 => 6,
                        70..=79 => 7,
                        80..=89 => 8,
                        _ => panic!("{}", ERR_DIGIT_TOO_LARGE),
                    };
                    *digit %= 10;
                });
                if carry > 0 {
                    T::from_iter(
                        std::iter::once((carry + TryInto::<u8>::try_into('0').unwrap()) as char)
                            .chain(
                                ret.into_iter()
                                    .map(|u| (u + TryInto::<u8>::try_into('0').unwrap()) as char),
                            ),
                    )
                } else {
                    T::from_iter(
                        ret.into_iter()
                            .map(|u| (u + TryInto::<u8>::try_into('0').unwrap()) as char),
                    )
                }
            }
            _ => panic!("{}", ERR_DIGIT_TOO_LARGE),
        }
    }
}

impl MyAddMul<String> for &str {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        Self::mul_worker(&num1, &num2)
    }
    fn mul_worker(s0: &str, s1: &str) -> String {
        let (s0, s1) = if s0.len() >= s1.len() {
            (s0, s1)
        } else {
            (s1, s0)
        };
        let mut ret = String::with_capacity(s0.len() + s1.len());
        ret.push('0');
        for (shift, u) in s1.chars().rev().enumerate() {
            let tmp = s0.shift_by_decimal(shift).as_str().mul_u(u as u8 - b'0');
            ret = tmp.as_str().add(&ret.as_str());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
