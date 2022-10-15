pub struct Solution;
use std::cell::RefCell;
use std::collections::BTreeSet as Tree;

const ERR_ASCII_LOW: &'static str = "Expect only ASCII lowercase";

/// ASCII lowercase equivalence class, where equivalence relation is defined
/// by that they are symmetric over shifting every char.
///
/// # Examples
/// ```
/// use lc_249_group_shifted_strings::*;
/// let s0 = "azk".to_string();
/// let s1 = "ihs".to_string();
/// assert!(<ShiftEqClass as From<String>>::from(s0.clone()).eq(&s1.clone().into()));
/// ```
#[derive(Debug)]
pub struct ShiftEqClass {
    // Records the actual occurences of strings in the input that's in
    // the EQ class.
    //
    // BTreeSet doesn't allow change to data since the invariants of the tree
    // may not be preserved if one mess around with something that causes order
    // to change, hence the interior mutability pattern.
    members: RefCell<Vec<String>>,
    representative: Vec<u8>,
}
impl PartialEq for ShiftEqClass {
    fn eq(&self, other: &Self) -> bool {
        self.representative.eq(&other.representative)
    }
}
impl PartialOrd for ShiftEqClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.representative.partial_cmp(&other.representative)
    }
}
impl Eq for ShiftEqClass {}
impl Ord for ShiftEqClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl ShiftEqClass {
    /// Allow only ASCII lowercase.
    pub fn new(s: String) -> Result<Self, &'static str> {
        if !s.chars().all(|c| c.is_ascii_lowercase()) {
            Err(ERR_ASCII_LOW)
        } else {
            const Z: u8 = 'z' as u8;
            const OFFSET: u8 = Z - 'a' as u8 + 1;
            Ok(Self {
                representative: Vec::from({
                    let u8_arr = <String as AsRef<[u8]>>::as_ref(&s);
                    u8_arr
                        .iter()
                        .map(|u| {
                            /*
                             * The strings "abc", "ijk", "xyz", and "zab" etc
                             * are in same equivalence class.
                             * Note we can choose a special representative for
                             * each class, e.g. the one with first letter
                             * being 'z', enabling easier comparison.
                             *
                             * One may think that it's possible to save some
                             * memory by skipping the first character since
                             * they are all 'z', but this would lead to
                             * confusion between the EQ class of "" and the EQ
                             * class of all single-char strings.
                             */
                            let mut ret = u + (Z - u8_arr[0]);
                            if ret > Z {
                                ret -= OFFSET;
                            }
                            ret
                        })
                        .collect::<Vec<u8>>()
                }),
                members: RefCell::new(vec![s]),
            })
        }
    }
    pub fn merge(&self, other: Self) -> Result<(), Self> {
        if !self.eq(&&other) {
            Err(other)
        } else {
            let mut members = self.members.borrow_mut();
            members.append(&mut other.members.borrow_mut());
            /*
             * TODO
             * In LC this problem is configured so stupid it expects length of
             * input to be same as the output,
             * i.e. duplicate is of no consideration.
             * However IMO this utility should also make unique.
             * Anyway enable these lines if one want aforementioned behavior.
             */
            // members.sort_unstable();
            // members.dedup();
            Ok(())
        }
    }
    pub fn get_members(self) -> Vec<String> {
        self.members.take()
    }
}
impl From<String> for ShiftEqClass {
    fn from(s: String) -> Self {
        Self::new(s).unwrap()
    }
}

/// We can shift a string by shifting each of its letters to its successive letter.
/// For example, "abc" can be shifted to be "bcd".
/// We can keep shifting the string to form a sequence.
/// For example, we can keep shifting "abc" to form the sequence:
/// "abc" -> "bcd" -> ... -> "xyz".
/// Given an array of strings strings, group all strings[i] that belong to the
/// same shifting sequence. You may return the answer in any order.
impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut tree: Tree<ShiftEqClass> = Tree::new();
        for s in strings {
            let s: ShiftEqClass = s.into();
            match tree.get(&s) {
                Some(shift_eq) => {
                    shift_eq.merge(s).unwrap();
                }
                None => {
                    tree.insert(s);
                }
            }
        }
        tree.into_iter()
            .map(|shift_eq| shift_eq.get_members())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::BTreeSet as Tree;
    #[test]
    fn test_soln() {
        let input = vec!["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"]
            .iter()
            .map(|&s| String::from(s))
            .collect::<Vec<_>>();
        let ans = vec![
            vec!["acef"],
            vec!["a", "z"],
            vec!["abc", "bcd", "xyz"],
            vec!["az", "ba"],
        ]
        .iter()
        .map(|vec_of_str| {
            vec_of_str
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Tree<_>>();
        let output = Solution::group_strings(input)
            .into_iter()
            .collect::<Tree<_>>();
        assert!(output
            .symmetric_difference(&ans)
            .cloned()
            .collect::<Vec<_>>()
            .is_empty());

        let input = vec!["a"]
            .iter()
            .map(|&s| String::from(s))
            .collect::<Vec<_>>();
        let ans = vec![vec!["a"]]
            .iter()
            .map(|vec_of_str| {
                vec_of_str
                    .iter()
                    .map(|&s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Tree<_>>();
        let output = Solution::group_strings(input)
            .into_iter()
            .collect::<Tree<_>>();
        assert!(output
            .symmetric_difference(&ans)
            .cloned()
            .collect::<Vec<_>>()
            .is_empty())
    }
}
