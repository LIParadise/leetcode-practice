pub struct Solution;

impl Solution {
    /// Input may not be sorted
    /// Return the max gap between neighboring elements if one were to sort the input array
    /// Shall run in linear time and space
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        let mut output = vec![0; nums.len()];
        Self::maximum_gap_worker(&mut nums, &mut output)
    }

    /// Input criteria:
    /// 1. `arr` and `workspace` are of same length
    /// Not conforming to the above criteria(s) results in unexpected behaviors
    ///
    /// Result:
    /// `arr` contains sorted result, whereas `workspace` contains garbage
    /// (acktually, partially sorted result in last radix sort iteration)
    ///
    /// Return:
    /// max gap between neighbors in the sorted array `arr`
    fn maximum_gap_worker(arr: &mut [i32], workspace: &mut [i32]) -> i32 {
        Self::radix_sort(arr, workspace);

        arr.iter()
            .zip(arr.iter().skip(1))
            .max_by(|(&a0, &a1), (&b0, &b1)| (a1 - a0).cmp(&(b1 - b0)))
            .map(|(a, b)| b - a)
            .unwrap_or(0)
    }

    /// Input criteria:
    /// 1. the function pointer returns radix
    /// 2. `counters` have enough length for all radix
    /// 3. `arr` and `workspace` are of same length
    /// Not conforming to the above criteria(s) results in unexpected behaviors
    ///
    /// Result:
    /// `workspace` contains sorted result
    fn counting_sort<F: Fn(i32) -> u8>(
        counters: &mut [usize],
        arr: &[i32],
        workspace: &mut [i32],
        lens: F,
    ) {
        counters.iter_mut().for_each(|u| *u = 0);
        arr.iter().for_each(|&u| {
            counters[lens(u) as usize] += 1;
        });
        let _ = counters.iter_mut().fold(0, |partial_sum, u| {
            *u += partial_sum;
            *u
        });
        arr.iter().rev().for_each(|&u| {
            counters[lens(u) as usize] -= 1;
            workspace[counters[lens(u) as usize]] = u;
        });
    }

    /// Input criteria:
    /// 1. `arr` and `workspace` are of same length
    /// Not conforming to the above criteria(s) results in unexpected behaviors
    ///
    /// Result:
    /// `arr` contains sorted result, whereas `workspace` contains garbage
    /// (acktually, partially sorted result in last radix sort iteration)
    fn radix_sort<'a>(mut arr: &'a mut [i32], mut workspace: &'a mut [i32]) {
        let mut counters = vec![0_usize; std::u8::MAX as usize + 1];
        static SIGN_BIT: i32 = 0x8000_0000_u32 as i32;
        static LOWEST_BYTE: i32 = 0x0000_00FF;
        // https://stackoverflow.com/a/52472629/9933842
        // To deal with negative numbers in radix sort,
        // just flip the sign bit,
        // assuming 2's complement.
        arr.iter_mut().for_each(|u| *u = *u ^ SIGN_BIT);

        for i in 0..4 {
            Self::counting_sort(&mut counters, &arr, &mut workspace, |x| {
                ((x >> (i * 8)) & LOWEST_BYTE).try_into().unwrap()
            });
            // partial result is in `workspace`
            // switch them by aliasing s.t. no need to copy whole array
            //
            // since `i32` is 4 bytes, after for loop,
            // they resp. correspond to their original arrays.
            (arr, workspace) = (workspace, arr);
        }

        // Remember to flip them back
        arr.iter_mut().for_each(|u| *u = *u ^ SIGN_BIT);
    }

    #[allow(unused)]
    fn assert_sorted(arr: &[i32]) -> bool {
        arr.iter().zip(arr.iter().skip(1)).all(|(a, b)| a <= b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let nums = vec![3, 6, 9, 1];
        assert_eq!(Solution::maximum_gap(nums), 3);
        let nums = vec![10];
        assert_eq!(Solution::maximum_gap(nums), 0);
        let nums = vec![
            494767408, 862126209, 213511142, 768240025, 631263193, 839199222, 990848796, 214568815,
            540853864, 760724418, 980162605, 976743981, 168965760, 680875496, 256709197, 970953816,
            948680062, 347254784, 732201399, 786249847, 782805044, 40906641, 674241381, 784330934,
            175502610, 731105392, 424650824, 549764101, 986090032, 710542549, 249208107, 448419816,
            527708664, 158499246, 223421723, 114552457, 466978424, 821491411, 19614107, 115389497,
            902211438, 2644108, 744489871, 897895073, 372311214, 551142753, 933294894, 426217662,
            217504874, 983488406, 516890023, 426853110, 971124103,
        ];
        assert_eq!(Solution::maximum_gap(nums), 90545587);
    }

    #[test]
    fn test_counting_sort() {
        let mut array = vec![3, 1, 4, 1, 5, 2, 7, 1, 8];
        let mut array = &mut array;
        let mut workspace = vec![0; array.len()];
        let mut workspace = &mut workspace;
        let mut counters = vec![0; 10];
        Solution::counting_sort(&mut counters, &array, &mut workspace, |x| x as u8);
        (array, workspace) = (workspace, array);
        assert_eq!(array, &mut vec![1, 1, 1, 2, 3, 4, 5, 7, 8]);

        array.clear();
        workspace.clear();
        Solution::counting_sort(&mut counters, &array, &mut workspace, |x| x as u8);
        array = workspace;
        assert_eq!(array, &mut vec![]);
    }

    #[test]
    fn test_radix_sort() {
        let mut array = vec![114514, 3, 1, 4, 1, 5, 9, 2, 6, 42069];
        let mut workspace = vec![0; array.len()];
        Solution::radix_sort(&mut array, &mut workspace);
        assert_eq!(array, vec![1, 1, 2, 3, 4, 5, 6, 9, 42069, 114514]);

        let mut array = vec![-114514, 3, 1, 4, 1, 5, -9, 2, 6, 42069];
        let mut workspace = vec![0; array.len()];
        Solution::radix_sort(&mut array, &mut workspace);
        assert_eq!(array, vec![-114514, -9, 1, 1, 2, 3, 4, 5, 6, 42069]);

        let mut array = vec![];
        let mut workspace = vec![];
        Solution::radix_sort(&mut array, &mut workspace);
        assert_eq!(array, vec![]);
    }
}
