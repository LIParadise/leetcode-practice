struct Solution;

struct RomanCheatCode {
    symbol: &'static str,
    value: i32,
}

static ROMAN_CHEATCODES: &[RomanCheatCode] = &[
    RomanCheatCode {
        symbol: "M",
        value: 1000,
    },
    RomanCheatCode {
        symbol: "CM",
        value: 900,
    },
    RomanCheatCode {
        symbol: "D",
        value: 500,
    },
    RomanCheatCode {
        symbol: "CD",
        value: 400,
    },
    RomanCheatCode {
        symbol: "C",
        value: 100,
    },
    RomanCheatCode {
        symbol: "XC",
        value: 90,
    },
    RomanCheatCode {
        symbol: "L",
        value: 50,
    },
    RomanCheatCode {
        symbol: "XL",
        value: 40,
    },
    RomanCheatCode {
        symbol: "X",
        value: 10,
    },
    RomanCheatCode {
        symbol: "IX",
        value: 9,
    },
    RomanCheatCode {
        symbol: "V",
        value: 5,
    },
    RomanCheatCode {
        symbol: "IV",
        value: 4,
    },
    RomanCheatCode {
        symbol: "I",
        value: 1,
    },
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut remainder = num;
        ROMAN_CHEATCODES
            .iter()
            .fold(String::with_capacity(10), |mut ret, cheat_code| {
                while remainder >= cheat_code.value {
                    ret.push_str(cheat_code.symbol);
                    remainder -= cheat_code.value;
                }
                ret
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(33), "XXXIII");
        assert_eq!(Solution::int_to_roman(34), "XXXIV");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
