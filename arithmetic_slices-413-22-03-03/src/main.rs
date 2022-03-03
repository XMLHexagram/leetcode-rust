fn main() {
    println!("Hello, world!");
}

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut i = 0;

    while i < nums.len() {
        let mut j = i + 1;
        while j < nums.len() - 1 {
            if nums[j] - nums[j - 1] == nums[j + 1] - nums[j] {
                j += 1
            } else {
                break;
            }
        }

        let length = j - i + 1 - 2;

        if length > 0 {
            count += length * (length + 1) / 2;
        }

        i = j // why?
    }

    count as i32
}

#[cfg(test)]
mod tests {
    use crate::number_of_arithmetic_slices;

    #[test]
    fn test_solution_1() {
        assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3]), 1);
        assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4, 5]), 6);
    }
}
