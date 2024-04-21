pub struct Solution;

use std::marker::PhantomData;

const ERR_MODULO_2: &str = "%2 gives neither 0 nor 1, abort.";

#[derive(Debug)]
pub struct MinHeap<T: Ord + Eq> {
    arr: Vec<T>,
    // TODO iterators
}

impl<T: Eq + Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.arr.len()
    }
    pub fn insert(&mut self, t: T) {
        self.arr.push(t);
        Self::float_up(self, self.arr.len() - 1);
    }
    pub fn min(&self) -> Option<&T> {
        self.arr.first()
    }
    pub fn pop_min(&mut self) -> Option<T> {
        self.remove_at(0)
    }
    fn remove_at(&mut self, i: usize) -> Option<T> {
        let last_idx = self.arr.len() - 1;
        self.arr.swap(i, last_idx);
        let ret = self.arr.pop();
        if i != last_idx {
            if let Some(parent) = Self::parent(i) {
                if self.arr[i].lt(&self.arr[parent]) {
                    self.float_up(i);
                    return ret;
                }
            }
            Self::heapify_down(self, i);
        }
        ret
    }
    pub fn get_by<F>(&self, t: &T, f: F) -> Option<&T>
    where
        F: FnMut(&T, &T) -> bool,
    {
        self.find_by(t, f).and_then(|i| self.arr.get(i))
    }
    fn find_by<F>(&self, t: &T, mut f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> bool,
    {
        self.arr
            .as_slice()
            .iter()
            .enumerate()
            .find_map(|(i, elem)| if f(t, elem) { Some(i) } else { None })
    }
    pub fn remove_by<F>(&mut self, t: &T, f: F) -> Option<T>
    where
        F: FnMut(&T, &T) -> bool,
    {
        Self::find_by(self, t, f).and_then(|i| self.remove_at(i))
    }
    pub fn modify_key_by<F>(&mut self, old: &T, new: T, f: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> bool,
    {
        Self::find_by(self, old, f).map(|i| {
            match new.cmp(old) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    self.arr[i] = new;
                    Self::float_up(self, i)
                }
                std::cmp::Ordering::Greater => {
                    self.arr[i] = new;
                    Self::heapify_down(self, i)
                }
            }
            i
        })
    }
    fn float_up(&mut self, mut u: usize) {
        let arr = &mut self.arr;
        while let Some(p) = Self::parent(u) {
            if arr[u].lt(&arr[p]) {
                arr.swap(u, p);
                u = p
            } else {
                break;
            }
        }
    }
    fn heapify_down(&mut self, mut u: usize) {
        loop {
            let l = Self::left_child(u);
            let r = Self::right_child(u);
            let mut extremal_idx = u;
            if let Some(child) = self.arr.get(l) {
                if child.lt(&self.arr[extremal_idx]) {
                    extremal_idx = l;
                }
            } else {
                break;
            }
            if let Some(child) = self.arr.get(r) {
                if child.lt(&self.arr[extremal_idx]) {
                    extremal_idx = r;
                }
            }
            if extremal_idx != u {
                self.arr.swap(u, extremal_idx);
                u = extremal_idx;
            } else {
                break;
            }
        }
    }
    fn parent(u: usize) -> Option<usize> {
        if u > 0 {
            Some((u - 1) / 2)
        } else {
            None
        }
    }
    fn left_child(u: usize) -> usize {
        u * 2 + 1
    }
    fn right_child(u: usize) -> usize {
        u * 2 + 2
    }
}

impl<T: Ord + Eq + Clone> From<&[T]> for MinHeap<T> {
    fn from(value: &[T]) -> Self {
        let mut ret = Self {
            arr: value.to_vec(),
        };
        (0..value.len() / 2).rev().for_each(|i| {
            ret.heapify_down(i);
        });
        ret
    }
}

#[derive(Clone, Debug)]
struct CounterWithIndex {
    i: usize,
    cnt: usize,
}

impl PartialEq for CounterWithIndex {
    fn eq(&self, other: &Self) -> bool {
        self.cnt.eq(&other.cnt)
    }
}
impl PartialOrd for CounterWithIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cnt.partial_cmp(&other.cnt)
    }
}
impl Eq for CounterWithIndex {}
impl Ord for CounterWithIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cnt.cmp(&other.cnt)
    }
}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let mana = Self::manacher(s);
        let mut min_cut_dp = MinHeap::from([CounterWithIndex { i: 0, cnt: 0 }].as_slice());
        (1..s.len()).for_each(|last| {
            let mut stack = Vec::new();
            while let Some(prev_min_cut) = min_cut_dp.pop_min() {
                if Self::is_palindrome(prev_min_cut.i + 1, last, mana.1.as_slice()) {
                    stack.into_iter().for_each(|x| min_cut_dp.insert(x));
                    min_cut_dp.insert(CounterWithIndex {
                        i: last,
                        cnt: prev_min_cut.cnt + 1,
                    });
                    break;
                } else {
                    stack.push(prev_min_cut);
                }
            }
            if min_cut_dp.len() != last + 1 {
                panic!("DP not gathering required info on subproblem, abort.");
            }
        });
        dbg!(&min_cut_dp);
        min_cut_dp
            .get_by(
                &CounterWithIndex {
                    i: s.len() - 1,
                    cnt: 0,
                },
                |c0: &CounterWithIndex, c1: &CounterWithIndex| c0.i == c1.i,
            )
            .map_or(0, |c| c.cnt)
            .try_into()
            .unwrap()
    }

    /// Manacher's algorithm.
    /// Given array, return maximal len of palindromes.
    /// Result len is 2 times that of input plus 1 for palindromes with even lenlth.
    // (2*l-1) suffices but (2*l+1) abstracts away some corner cases
    fn manacher(s: &[u8]) -> (PhantomData<&()>, Vec<usize>) {
        let mut radii = vec![0; s.len() * 2 + 1];
        // these two are based on index of return value, rather than input array
        let mut center = 0;
        let mut radius = 0;
        while center < radii.len() {
            let (l, r) = s.split_at(center / 2);
            radius += match center % 2 {
                0 => {
                    // For even palindromes, i.e. considering center be at some hidden symbol
                    r.iter()
                        .skip(radius / 2)
                        .zip(l.iter().rev().skip(radius / 2))
                        .take_while(|(l, r)| l == r)
                        .count()
                        * 2
                }
                1 => {
                    // For odd palindromes,
                    // i.e. considering center be at some concrete element of input
                    r.iter()
                        .skip(radius / 2 + 1)
                        .zip(l.iter().rev().skip(radius / 2))
                        .take_while(|(l, r)| l == r)
                        .count()
                        * 2
                    // remember to account for Manacher's hidden symbol
                        + if radius == 0 { 1 } else { 0 }
                }
                _ => panic!("{}", ERR_MODULO_2),
            };

            radii[center] = radius;
            let old_radius = radius;
            let old_center = center;
            center += 1;
            radius = 0;

            while center <= old_center + old_radius {
                let mirrored_center = old_center - (center - old_center);
                let mirror_radius_bound = old_center + old_radius - center;
                match radii[mirrored_center].cmp(&mirror_radius_bound) {
                    std::cmp::Ordering::Less => {
                        radii[center] = radii[mirrored_center];
                        center += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        radii[center] = mirror_radius_bound;
                        center += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        radius = mirror_radius_bound;
                        break;
                    }
                }
            }
        }
        (PhantomData, radii)
    }

    /// Given concrete indices and Manacher array (including the interstices)
    /// return if it's palindrome
    fn is_palindrome(first: usize, last: usize, mana: &[usize]) -> bool {
        (last - first + 1) <= mana[first + last + 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::{MinHeap, Solution};
    #[test]
    fn test_soln() {
        assert_eq!(Solution::min_cut(String::from("a")), 0);
        assert_eq!(Solution::min_cut(String::from("ab")), 1);
        assert_eq!(Solution::min_cut(String::from("aab")), 1);
        assert_eq!(Solution::min_cut(String::from("aaab")), 1);
        assert_eq!(Solution::min_cut(String::from("ababab")), 1);
        assert_eq!(Solution::min_cut(String::from("abababa")), 0);
    }

    #[test]
    fn test_manacher() {
        let arr = &[];
        let m = Solution::manacher(arr);
        assert_eq!(&[0], m.1.as_slice());

        let arr = &[0];
        let m = Solution::manacher(arr);
        assert_eq!(&[0, 1, 0], m.1.as_slice());

        let arr = &[0, 0];
        let m = Solution::manacher(arr);
        assert_eq!(&[0, 1, 2, 1, 0], m.1.as_slice());

        let arr = &[0, 0, 0];
        let m = Solution::manacher(arr);
        assert_eq!(&[0, 1, 2, 3, 2, 1, 0], m.1.as_slice());

        let arr = &[0, 1, 2];
        let m = Solution::manacher(arr);
        assert_eq!(&[0, 1, 0, 1, 0, 1, 0], m.1.as_slice());

        let arr = &[0, 1, 0];
        let m = Solution::manacher(arr);
        assert_eq!(&[0, 1, 0, 3, 0, 1, 0], m.1.as_slice());
    }

    #[test]
    fn test_min_heap() {
        let arr = &[3, 1, 4, 1, 5, 9];
        let mut heap = MinHeap::from(arr.as_slice());
        let mut ans = arr.to_vec();
        ans.sort_unstable();
        ans.into_iter().for_each(|x| {
            assert_eq!(x, heap.pop_min().unwrap());
        });

        let arr = &[0_usize; 0];
        let mut heap = MinHeap::from(arr.as_slice());
        let mut ans = arr.to_vec();
        ans.sort_unstable();
        ans.into_iter().for_each(|x| {
            assert_eq!(x, heap.pop_min().unwrap());
        });

        let arr = &[2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9];
        let mut heap = MinHeap::from(arr.as_slice());
        assert!(heap.modify_key_by(&1, 10, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&1, 10, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&1, 10, |i, j| i == j).is_none());
        let mut ans = arr.to_vec();
        while let Some(i) = ans.iter().position(|x| x == &1) {
            ans[i] = 10;
        }
        ans.sort_unstable();
        ans.into_iter().for_each(|x| {
            assert_eq!(x, heap.pop_min().unwrap());
        });

        let arr = &[2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9];
        let mut heap = MinHeap::from(arr.as_slice());
        assert!(heap.modify_key_by(&1, 10, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&1, 10, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&1, 10, |i, j| i == j).is_none());
        assert!(heap.modify_key_by(&8, 6, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&8, 7, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&9, 3, |i, j| i == j).is_some());
        assert!(heap.modify_key_by(&2, 1, |i, j| i == j).is_some());
        let mut ans = vec![2, 7, 10, 6, 2, 7, 10, 8, 1, 8, 4, 5, 3];
        ans.sort_unstable();
        ans.into_iter().for_each(|x| {
            assert_eq!(x, heap.pop_min().unwrap());
        });
    }
}
