pub struct Solution;

impl Solution {
    pub fn max_product(s: String) -> i32 {
        todo!()
    }
    fn manacher(arr: &[u8]) -> Option<&[u8]> {
        let mut center = 0;
        let mut radius = 0;
        let mut radii = vec![0; arr.len() * 2 + 1];
        while center < radii.len() {
            let (l, r) = arr.split_at(center / 2);
            radius += if center % 2 == 1 {
                l.iter()
                    .rev()
                    .zip(r.iter().skip(1))
                    .take_while(|(a, b)| a == b)
                    .count()
                    * 2
                    + if radius == 0 {
                        // Manacher's algorithm hidden symbol
                        1
                    } else {
                        // if radius ain't 0, the copy part of Manacher's algorithm already took care of
                        // the extra hidden special symbol, so no need to add them
                        0
                    }
            } else {
                l.iter()
                    .rev()
                    .zip(r.iter())
                    .take_while(|(a, b)| a == b)
                    .count()
                    * 2
            };
            radii[center] = radius;
            let old_center = center;
            let old_radius = radius;
            center += 1;
            radius = 0;
            while center <= old_center + old_radius {
                let mirrored_center = old_center - (center - old_center);
                let mirror_radius_bound = old_center + old_radius - center;
                if radii[mirrored_center] < mirror_radius_bound {
                    radii[center] = radii[mirrored_center];
                    center += 1;
                } else if radii[mirrored_center] > mirror_radius_bound {
                    radii[center] = mirror_radius_bound;
                    center += 1;
                } else {
                    radius = mirror_radius_bound;
                    break;
                }
            }
        }
        radii
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, radius)| &arr[(i - radius) / 2..(i + radius + 1) / 2])
    }
    fn is_palindrome(arr: &[u8]) -> bool {
        arr.iter()
            .enumerate()
            .zip(arr.iter().enumerate().rev())
            .take_while(|((i, _), (j, _))| j > i)
            .all(|((_, a), (_, b))| a == b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {}
    #[test]
    fn test_manacher() {
        let arr = [1, 2, 3, 2, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some(arr.as_slice()));
        let arr = [];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some(arr.as_slice()));
        let arr = [1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some(arr.as_slice()));
        let arr = [1, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some(arr.as_slice()));
        let arr = [1, 2];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([2].as_slice()));
        let arr = [1, 1, 2];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([1, 1].as_slice()));
        let arr = [2, 1, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([1, 1].as_slice()));
        let arr = [1, 1, 2, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([1, 2, 1].as_slice()));
        let arr = [1, 2, 1, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([1, 2, 1].as_slice()));
        let arr = [3, 1, 1, 2, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([1, 2, 1].as_slice()));
        let arr = [2, 1, 1, 2, 1];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([2, 1, 1, 2].as_slice()));
        let arr = [1, 2, 1, 1, 2];
        let arr = &arr;
        assert_eq!(Solution::manacher(arr), Some([2, 1, 1, 2].as_slice()));
    }
}
