fn main() {
    println!("Hello, world!");
}

/*

poured: sum of all the champagne poured
ol: left overflow
or: right overflow
sl: left speed
sr: right speed

     0,0 n-1

   1,0 1,1 n-1/2

  2,0 2,1 2,2 ((n-1)/2-1)/2

3,1 3,2 3,3 3,4

4,1 4,2 4,3 4,4 4,5

1,1
ol = 0 sl = 0
or = 1 sr = 1/2

2,2
ol = 3 sl = 1/4
or = 3 sr = 1/4

2,1
ol = 0 sl = 0
or = 3 sr = 1/4

3,2
ol = sl

c(m,n)
c(m,n) = (c(m-1,n)-1)/2 + (c(m-1,n-1)-1)/2

*/
// struct Solution {}

// c(0,0)=poured, m, n
// impl Solution {
//     pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
//         if query_glass > query_row {
//             return 1f64;
//         }

//         if query_row == 0 && query_glass == 0 {
//             return poured as f64;
//         }

//         return (Solution::champagne_tower(poured, query_row - 1, query_glass) - 1.0) / 2.0
//             + (Solution::champagne_tower(poured, query_row - 1, query_glass - 1) - 1.0) / 2.0;
//     }
// }

pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let query_row = query_row as usize;
    let query_glass = query_glass as usize;

    let mut state = [[0f64; 100]; 100];

    state[0][0] = poured as f64;
    for i in 1..query_row + 1 {
        for j in 0..i + 1 {
            let mut left;

            if j == 0 {
                left = 0.0;
            } else {
                left = state[i - 1][j - 1] - 1.0;
                if left < 0.0 {
                    left = 0.0;
                }
            }

            let mut right;
            if j == i {
                right = 0.0;
            } else {
                right = state[i - 1][j] - 1.0;
                if right < 0.0 {
                    right = 0.0;
                }
            }

            state[i][j] = (left / 2.0) + (right / 2.0)
        }
    }

    let res = if state[query_row as usize][query_glass as usize] > 1.0 {
        1.0
    } else {
        state[query_row as usize][query_glass as usize]
    };

    // println!("{:?}", state);x
    return res;
}

#[cfg(test)]
mod tests {
    use crate::champagne_tower;

    #[test]
    fn test_solution_1() {
        // assert_eq!(champagne_tower(1, 1, 1), 0.0);
        assert_eq!(champagne_tower(2, 1, 1), 0.5);
        assert_eq!(champagne_tower(100000009, 33, 17), 1.0);
        assert_eq!(champagne_tower(25, 6, 1), 0.18750)
    }
}
