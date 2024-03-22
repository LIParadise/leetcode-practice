pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = if a.len() >= b.len() {
            (a.as_bytes(), b.as_bytes())
        } else {
            (b.as_bytes(), a.as_bytes())
        };
        let mut actual_sum = Vec::new();
        let mut carry = 0;
        let u32_in_u8_slice = &mut [0; 4];
        static PADDING: &'static [u8] = &[0, 0, 0, 0];
        a.rchunks(4)
            .zip(b.rchunks(4).chain(std::iter::repeat(PADDING)))
            .for_each(|(a, b)| {
                u32_in_u8_slice.fill(0);
                u32_in_u8_slice[4 - a.len()..].clone_from_slice(a);
                let a = u32::from_be_bytes(*u32_in_u8_slice);
                u32_in_u8_slice.fill(0);
                u32_in_u8_slice[4 - b.len()..].clone_from_slice(b);
                let b = u32::from_be_bytes(*u32_in_u8_slice);
                match a.overflowing_add(b) {
                    (wrapped_sum, true) => {
                        // it's impossible for a wrapped result plus 1 to overflow again
                        // e.g. 1111 + 1111 = 1_1110
                        // and 1110 + 0001 = 1111
                        actual_sum.push(wrapped_sum + carry);
                        carry = 1;
                    }
                    (sum, false) => match sum.overflowing_add(carry) {
                        (_, true) => {
                            actual_sum.push(0);
                            carry = 1;
                        }
                        (sum, false) => {
                            actual_sum.push(sum);
                            carry = 0;
                        }
                    },
                }
            });
        if carry == 1 {
            actual_sum.push(1);
        }
        static U32_MSB: u32 = 0x8000_0000;
        if actual_sum.len() >= 1 {
            let mut ret = actual_sum
                .pop()
                .map(|mut u| {
                    let mut bits_remained = 32;
                    while u & U32_MSB == 0 {
                        u <<= 1;
                        bits_remained -= 1;
                    }
                    (0..bits_remained)
                        .map(|_| {
                            let b = if u & U32_MSB == U32_MSB {
                                '1' as u8
                            } else {
                                '0' as u8
                            };
                            u <<= 1;
                            b
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap();
            ret.extend(actual_sum.into_iter().rev().flat_map(|mut u| {
                (0..32).map(move |_| {
                    let b = if u & U32_MSB == U32_MSB {
                        '1' as u8
                    } else {
                        '0' as u8
                    };
                    u <<= 1;
                    b
                })
            }));
            String::from_utf8(ret).unwrap()
        } else {
            String::new()
        }
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
