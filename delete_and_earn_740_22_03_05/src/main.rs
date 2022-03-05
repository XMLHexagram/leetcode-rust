fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

// impl Solution {
//     pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
//         let mut nums = nums;
//         let mut res = 0;
//         let mut i = 0;

//         nums.sort_by(|a, b| b.cmp(&a));

//         loop {
//             let ii = nums.get(i);
//             if let Some(&ii) = ii {
//                 let small = nums.iter().filter(|&&x| x == ii - 1);
//                 let big = nums.iter().filter(|&&x| x == ii + 1);
//                 let iii = nums.iter().filter(|&&x| x == ii);
//                 let sum: i32 = small.sum::<i32>() + big.sum::<i32>();
//                 if sum > iii.sum::<i32>() {
//                     i += 1;
//                     continue;
//                 }
//                 res += ii;
//                 nums.swap_remove(i);
//                 nums.retain(|&x| x != ii - 1);
//                 nums.retain(|&x| x != ii + 1);
//             } else {
//                 if i == 0 {
//                     return res;
//                 }
//                 i = 0;
//             }
//         }
//     }
// }

use std::{collections::HashMap, hash::Hash};
// pub struct Solution {
//     points: HashMap<i32, i32>,
// }


impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut points = HashMap::new();
        let mut cache: HashMap<i32, i32> = HashMap::new();

        let mut max = 0;

        for ele in nums {
            let res = points.get(&ele);
            match res {
                Some(&i) => points.insert(ele, ele + i),
                _ => points.insert(ele, ele),
            };
            if ele > max {
                max = ele;
            }
        }

        return Solution::max1(max, &points, &mut cache);
    }

    pub fn max1(num: i32, points: &HashMap<i32, i32>, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&res) = cache.get(&num) {
            return res;
        }

        if num <= 0 {
            return 0;
        }

        let res = points.get(&num);
        match res {
            Some(&i) => {
                let res = (i + Solution::max1(num - 2, points, cache)).max(Solution::max1(
                    num - 1,
                    points,
                    cache,
                ));
                cache.insert(num, res);
                return res;
            }
            None => {
                let res = (0 + Solution::max1(num - 2, points, cache)).max(Solution::max1(
                    num - 1,
                    points,
                    cache,
                ));
                cache.insert(num, res);
                return res;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_solution_1() {
        // let mut solution = Solution {
        //     points: HashMap::new(),
        // };
        // assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        // assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
        // assert_eq!(
        //     Solution::delete_and_earn(vec![
        //         8, 3, 4, 7, 6, 6, 9, 2, 5, 8, 2, 4, 9, 5, 9, 1, 5, 7, 1, 4
        //     ]),
        //     61
        // );
        assert_eq!(
            Solution::delete_and_earn(vec![
                12, 32, 93, 17, 100, 72, 40, 71, 37, 92, 58, 34, 29, 78, 11, 84, 77, 90, 92, 35,
                12, 5, 27, 92, 91, 23, 65, 91, 85, 14, 42, 28, 80, 85, 38, 71, 62, 82, 66, 3, 33,
                33, 55, 60, 48, 78, 63, 11, 20, 51, 78, 42, 37, 21, 100, 13, 60, 57, 91, 53, 49,
                15, 45, 19, 51, 2, 96, 22, 32, 2, 46, 62, 58, 11, 29, 6, 74, 38, 70, 97, 4, 22, 76,
                19, 1, 90, 63, 55, 64, 44, 90, 51, 36, 16, 65, 95, 64, 59, 53, 93
            ]),
            61
        )
    }
}
