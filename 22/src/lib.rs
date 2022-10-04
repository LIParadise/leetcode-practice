pub struct Solution;

/// Given n pairs of parentheses, write a function to generate all combinations
/// of well-formed parentheses.
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut memory = Vec::new();
        if n > 8 || n < 0 {
            Vec::new()
        } else {
            let n = n as usize;
            Self::gen_paren_worker(&mut memory, n);
            memory.pop().unwrap()
        }
    }
    fn gen_paren_worker(memory: &mut Vec<Vec<String>>, n: usize) {
        if memory.is_empty() {
            memory.push(vec!["".to_string()]);
        }
        use std::collections::BTreeSet as Tree;
        /*
         * Base cases:
         * 1 => ()
         * 2 => ()(), (())
         * 3 => ((())), (()()), (())(), ()(()), ()()()
         *
         * Consider n
         * We consider (n-1) * (1)
         *             (n-2) * (2)
         *             (n-3) * (3)...
         * And finally ({n-1}) since it's not combination from simpler cases.
         *
         * Notice some strings are counted multiple times.
         */
        while memory.len() < n + 1 {
            // Construct set of combinations using `.len()` parentheses
            let mut set = Tree::new();
            /*
             * (n-1) * (1)
             * (n-2) * (2)
             * (n-3) * (3)...
             */
            for i in 1..memory.len() {
                for s0 in &memory[i] {
                    for s1 in &memory[memory.len() - i] {
                        set.insert(String::from(s0) + s1);
                    }
                }
            }
            // ({n-1})
            for s in memory.last().unwrap() {
                set.insert('('.to_string() + s + &')'.to_string());
            }
            memory.push(set.into_iter().collect());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let ans = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(Solution::generate_parenthesis(3), ans);
    }
}
