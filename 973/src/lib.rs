pub struct Solution;

#[inline]
fn euc_norm(x: i32, y: i32) -> i32 {
    x * x + y * y
}
struct Point<'a> {
    pub coordinate: &'a Vec<i32>,
    norm: i32,
}
impl<'a> Point<'a> {
    pub fn new(coordinate: &'a Vec<i32>) -> Self {
        Self {
            coordinate,
            norm: euc_norm(coordinate[0], coordinate[1]),
        }
    }
}
impl<'a> PartialEq for Point<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.norm.eq(&other.norm)
    }
}
impl<'a> PartialOrd for Point<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.norm.partial_cmp(&other.norm)
    }
}
impl<'a> Eq for Point<'a> {}
impl<'a> Ord for Point<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.norm.cmp(&other.norm)
    }
}

impl Solution {
    /// Given an array of points where points[i] = [xi, yi] represents a point
    /// on the X-Y plane and an integer k, return the k closest points to the
    /// origin (0, 0).
    // Idea: max heap
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap as Heap;
        let k = k as usize;
        let mut ret = Heap::with_capacity(k);
        points.iter().for_each(|p| {
            if ret.len() < k {
                ret.push(Point::new(p));
            } else {
                let p = Point::new(p);
                if let std::cmp::Ordering::Less = p.cmp(ret.peek().unwrap()) {
                    ret.pop();
                    ret.push(p);
                }
            }
        });
        ret.iter().map(|p| p.coordinate.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
