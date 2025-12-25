/*
2
100
10000
*/

// use std::io::{self, BufRead};

// fn main() {
//     let stdin = io::stdin();
//     let mut stdin_iter = stdin.lock().lines();

//     let t = stdin_iter.next().unwrap().unwrap().parse::<i32>().unwrap();

//     let mut v: Vec<i32> = vec![];
//     for _ in 0..t {
//         let n = stdin_iter.next().unwrap().unwrap().parse::<i32>().unwrap();
//         v.push(n);
//     }

//     let mut res: i32 = 0;
//     for num in &v {
//         println!("{}", num);
//         res += num;
//     }

//     println!("{}", res);
// }

/*
20 2
*/

// use std::io::{self, BufRead};

// fn main() {
//     let stdin = io::stdin();
//     let mut stdin_iter = stdin.lock().lines();

//     let nums: Vec<_> = stdin_iter.next().unwrap().unwrap()
//                         .trim()
//                         .split(' ')
//                         .map(|s| s.parse::<i32>().unwrap())
//                         .collect();
//     for num in &nums {
//         println!("{}", num);
//     }
// }

/*
2
10 5
3675356291
10 5
2709360626
*/

// use std::io::{self, BufRead};

// fn main() {
//     let stdin = io::stdin();
//     let mut stdin_iter = stdin.lock().lines();

//     let n = stdin_iter.next().unwrap().unwrap().parse::<i32>().unwrap();

//     for i in 0..n {
//         let nums: Vec<_> = stdin_iter.next().unwrap().unwrap()
//                                         .trim()
//                                         .split(' ')
//                                         .map(|s| s.to_string())
//                                         .collect();
//         let digits = stdin_iter.next().unwrap().unwrap().parse::<String>().unwrap();

//         println!("{}, {}, {}", nums[0], nums[1], digits);
//         if digits.len() != nums[0].parse::<usize>().unwrap() {
//             println!("Digits is wrong");
//         } else {
//             println!("Get correct input");
//         }
//     }
// }

/*
Input consists of n lines each containing n integers.
*/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    
    let mut stdin_iter = stdin.lock().lines();

    let n = stdin_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
    
    let mut grid :Vec<Vec<_>> = Vec::with_capacity(n);

    for i in 0..n {
        
        let v = stdin_iter.next().unwrap().unwrap()
                                    .trim()
                                    .split(' ')
                                    .map(|s| s.to_string().parse::<i32>().unwrap())
                                    .collect();
        
        grid.push(v);
    }

    println!("[Result]");

    for i in 0..n {
        for j in 0..n {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}