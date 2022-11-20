fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        // let mut digits = digits.clone();

        for i in (0..digits.len()).rev() {
            digits[i] += 1;
            if digits[i] == 10 {
                digits[i] = 0;
                if i == 0 {
                   digits.insert(0, 1)
                }
            } else {
                break;
            }
        }  
        return digits;
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::plus_one([9].to_vec()),[1,0].to_vec());
        assert_eq!(Solution::plus_one([1,2,3].to_vec()),[1,2,4].to_vec());
    }
}