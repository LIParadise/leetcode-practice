pub struct Solution;

impl Solution {
    pub fn max_product(s: String) -> i64 {
        let s = s.as_bytes();

        // What are all maximal odd length palindromes?
        // This array stores such info:
        // suppose at index i we have l,
        // it means [i-l..i] and [i+1..=i+l] are mirror,
        // and it's a palindrome with length (2*i+1).
        //
        // The gist is to do this in linear time and space,
        // using Manacher's Algorithm
        //
        // [Fluent Algorithms on YouTube](https://www.youtube.com/watch?v=YVZttWzvyw8)
        // [Wikipedia pseudo code](https://en.wikipedia.org/wiki/Longest_palindromic_substring)
        let mut symmetric_len = vec![0; s.len()];
        {
            // Manacher's Algorithm, modified to consider only odd length palindromes
            // Modification is as simple as NOT inserting special characters
            let mut center = 0;
            let mut radius = 0;
            while center < s.len() {
                let (left, right) = s.split_at(center);
                left.iter()
                    .rev()
                    .skip(radius)
                    .zip(right.iter().skip(radius + 1))
                    .take_while(|(a, b)| a == b)
                    .for_each(|_| radius += 1);
                symmetric_len[center] = radius;

                let old_center = center;
                let old_radius = radius;
                center += 1;
                radius = 0;

                while center <= old_radius + old_center {
                    let mirrored_center = old_center - (center - old_center);
                    let mirrored_radius_bound = old_center + old_radius - center;
                    if symmetric_len[mirrored_center] < mirrored_radius_bound {
                        symmetric_len[center] = symmetric_len[mirrored_center];
                        center += 1;
                    } else if symmetric_len[mirrored_center] > mirrored_radius_bound {
                        symmetric_len[center] = mirrored_radius_bound;
                        center += 1;
                    } else {
                        radius = mirrored_radius_bound;
                        break;
                    }
                }
            }
        }

        // Suppose at index i, what max palindrome len is possible towards left and right?
        // Self included,
        // so for example consider a string consisting of only 1 char with len 10,
        // at index 0, it stores 1 to the left and 10 to the right
        // at index 9, it stores 10 to the left and 1 to the right
        //
        // This array answers the question.
        //
        // .0 stores max to the left (self included)
        // .1 stores max to the right (self included)
        let mut left_right = vec![(0, 0); s.len()];

        // Consider the max palindrome len to the left of each char (self included)
        // Observation:
        // 1. it's non-decreasing as we traverse down the string
        // 2. the exact indices it increased must due to those indices correspond to an unprecedented palindrome
        // 3. we do have required info for 2.: the maxial odd palindromes we just calculated.
        //
        // Here we do them locally, i.e. for each new record length, 
        // suppose it's of length (2 * l + 1) (remember we're dealing only with odd palindrome)
        // update only those l indices to the right.
        //
        // Finally, remember to also do the same for (max palindrome len to the right)
        {
            symmetric_len
                .iter()
                .scan(None, |longest_till_here, &x| {
                    Some(match longest_till_here {
                        None => true,
                        Some(y) if x > *y => {
                            *longest_till_here = Some(x);
                            true
                        }
                        _ => false,
                    })
                })
                .zip(symmetric_len.iter().enumerate())
                .for_each(|(new_longest, (i, &x))| {
                    if new_longest {
                        left_right[i..=i + x]
                            .iter_mut()
                            .enumerate()
                            .rev()
                            .try_for_each(|(l, y)| {
                                if 2 * l + 1 > y.0 {
                                    y.0 = 2 * l + 1;
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            })
                            .ok();
                    }
                });
            symmetric_len
                .iter()
                .rev()
                .scan(None, |longest_till_here, &x| {
                    Some(match longest_till_here {
                        None => true,
                        Some(y) if x > *y => {
                            *longest_till_here = Some(x);
                            true
                        }
                        _ => false,
                    })
                })
                .zip(symmetric_len.iter().enumerate().rev())
                .for_each(|(new_longest, (i, &x))| {
                    if new_longest {
                        left_right[i - x..=i]
                            .iter_mut()
                            .rev()
                            .enumerate()
                            .rev()
                            .try_for_each(|(l, y)| {
                                if 2 * l + 1 > y.1 {
                                    y.1 = 2 * l + 1;
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            })
                            .ok();
                    }
                });
        }

        #[cfg(test)]
        {
            println!("{:?}", s);
            println!("s{:?}", symmetric_len);
            println!("l{:?}", left_right.iter().map(|p| p.0).collect::<Vec<_>>());
            println!("r{:?}", left_right.iter().map(|p| p.1).collect::<Vec<_>>());
        }

        // In last block, we did only local updates
        // Here, we make the correct info is spreaded to whole array
        {
            (0..left_right.len()).for_each(|i| {
                let (a, b) = left_right.split_at_mut(i);
                if let Some(l) = a.last() {
                    b[0].0 = std::cmp::max(l.0, b[0].0);
                }
            });
            (0..left_right.len()).rev().for_each(|i| {
                let (a, b) = left_right.split_at_mut(i);
                if let Some(l) = a.last_mut() {
                    l.1 = std::cmp::max(l.1, b[0].1);
                }
            });
        }

        #[cfg(test)]
        {
            println!("l{:?}", left_right.iter().map(|p| p.0).collect::<Vec<_>>());
            println!("r{:?}", left_right.iter().map(|p| p.1).collect::<Vec<_>>());
        }

        // With all the info, it's as simple as a linear scan
        left_right
            .iter()
            .zip(left_right.iter().skip(1))
            .max_by(|(a, b), (c, d)| (a.0 * b.1).cmp(&(c.0 * d.1)))
            .map(|(a, b)| a.0 * b.1)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_product("ababbb".to_owned()), 9);
        assert_eq!(Solution::max_product("zaaaxbbby".to_owned()), 9);
        assert_eq!(
            Solution::max_product("ggbswiymmlevedhkbdhntnhdbkhdevelmmyiwsbgg".to_owned()),
            45
        );
        assert_eq!(
            Solution::max_product("wtbptdhbjqsrwkxccxkwrsqjbhdtpbtw".to_owned()),
            1
        );
    }
}
