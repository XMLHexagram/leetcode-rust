fn main() {
    println!("Hello, world!");
}

struct Solution;

const MOD:i128 = 1_000_000_007;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut res:i128 = 1;

        for i in 1..n+1 {
            let i:i128 = i as i128;
            res *= i;
            res *= (i-1)*2+1;
            res = res % MOD;
        }

        return res as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_solution_1() {
        assert_eq!(Solution::count_orders(7), 681080400);
        assert_eq!(Solution::count_orders(8), 729647433);
    }
}
