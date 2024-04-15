use regex::Regex;

pub struct Solution;

mod regex {
    use regex_internals::*;
    use std::{iter::Peekable, str::Chars};
    pub struct Regex<'a> {
        re: &'a str,
    }
    impl<'a> Regex<'a> {
        pub fn new(re: &'a str) -> Self {
            Self { re }
        }
        fn iter(&self) -> RegexParser<'a> {
            RegexParser {
                ptr: self.re.chars().peekable(),
            }
        }
        pub fn is_match(&self, s: &'a str) -> bool {
            self.is_match_worker(s.chars().peekable(), self.iter().peekable())
        }
        fn is_match_worker(
            &self,
            mut s_iter: Peekable<Chars<'a>>,
            mut re_iter: Peekable<RegexParser>,
        ) -> bool {
            use ReMatch::*;
            match (re_iter.peek(), s_iter.peek()) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(re), None) => match re {
                    SingleLower(_) | SingleWildcard => false,
                    ZeroOrMoreLower(_) | ZeroOrMoreWildcard => {
                        re_iter.next();
                        self.is_match_worker(s_iter, re_iter)
                    }
                },
                (Some(re), Some(s_ch)) => match re {
                    SingleLower(re_ch) if re_ch == s_ch => {
                        re_iter.next();
                        s_iter.next();
                        self.is_match_worker(s_iter, re_iter)
                    }
                    SingleLower(_) => false,
                    SingleWildcard => {
                        re_iter.next();
                        s_iter.next();
                        self.is_match_worker(s_iter, re_iter)
                    }
                    ZeroOrMoreLower(re_ch) if re_ch != s_ch => {
                        re_iter.next();
                        self.is_match_worker(s_iter, re_iter)
                    }
                    ZeroOrMoreLower(re_ch) => {
                        let how_many_same_chars = {
                            let s_iter = s_iter.clone();
                            s_iter.take_while(|ch| ch == re_ch).count()
                        };
                        re_iter.next();
                        (0..=how_many_same_chars).any(|idx| {
                            let mut s_iter = s_iter.clone();
                            for _ in 0..idx {
                                s_iter.next();
                            }
                            self.is_match_worker(s_iter, re_iter.clone())
                        })
                    }
                    ZeroOrMoreWildcard => {
                        re_iter.next();
                        let later_mandatory_re = {
                            let mut tmp = re_iter.clone();
                            let mut ret = None;
                            while tmp.peek().is_some() {
                                match tmp.peek().unwrap() {
                                    SingleLower(_) | SingleWildcard => {
                                        ret = Some(tmp);
                                        break;
                                    }
                                    _ => {}
                                }
                                tmp.next();
                            }
                            ret
                        };
                        match later_mandatory_re {
                            None => true,
                            Some(mut later_mandatory_re) => {
                                match later_mandatory_re.peek().unwrap() {
                                    SingleLower(re_ch) => {
                                        let mut possible_matches = Vec::new();
                                        {
                                            let mut possible_char_iter = s_iter.clone();
                                            while let Some(possible_char) =
                                                possible_char_iter.peek()
                                            {
                                                if possible_char == re_ch {
                                                    possible_matches
                                                        .push(possible_char_iter.clone())
                                                }
                                                possible_char_iter.next();
                                            }
                                        }
                                        possible_matches
                                            .into_iter()
                                            .any(|i| self.is_match_worker(i, re_iter.clone()))
                                    }
                                    SingleWildcard => {
                                        let mut possible_matches = Vec::new();
                                        {
                                            let mut possible_char_iter = s_iter.clone();
                                            while let Some(_) = possible_char_iter.peek() {
                                                possible_matches.push(possible_char_iter.clone());
                                                possible_char_iter.next();
                                            }
                                        }
                                        possible_matches
                                            .into_iter()
                                            .any(|i| self.is_match_worker(i, re_iter.clone()))
                                    }
                                    _ => panic!("???"),
                                }
                            }
                        }
                    }
                },
            }
        }
    }

    mod regex_internals {
        use std::{iter::Peekable, str::Chars};
        #[derive(Clone, Debug)]
        pub enum ReMatch {
            SingleLower(char),
            ZeroOrMoreLower(char),
            SingleWildcard,
            ZeroOrMoreWildcard,
        }
        #[derive(Clone, Debug)]
        pub struct RegexParser<'a> {
            pub(super) ptr: Peekable<Chars<'a>>,
        }
        impl<'a> Iterator for RegexParser<'a> {
            type Item = ReMatch;
            fn next(&mut self) -> Option<Self::Item> {
                use ReMatch::*;
                if let Some(ch) = self.ptr.next() {
                    Some(match (ch, self.ptr.peek()) {
                        ('a'..='z', Some(&'*')) => {
                            self.ptr.next();
                            ZeroOrMoreLower(ch)
                        }
                        ('a'..='z', _) => SingleLower(ch),
                        ('.', Some(&'*')) => {
                            self.ptr.next();
                            ZeroOrMoreWildcard
                        }
                        ('.', _) => SingleWildcard,
                        _ => panic!("Parse RE err"),
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Solution {
    /// Basic Regex support.
    /// Only [[:lower:].*], i.e. lowercases, '.', and '*'.
    pub fn is_match(s: String, p: String) -> bool {
        let regex = Regex::new(&p);
        regex.is_match(&s)
    }
}

#[cfg(test)]
mod tests;
