//mod p1;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut std_iter = stdin.lock().lines();
    
    let mut test_case = 0;
    if let Some(value) = std_iter.next() {
        match value {
            Ok(tc) => {
                let tc_p = tc.parse::<u32>();
                match tc_p {
                    Ok(tc_p) => test_case = tc_p,
                    Err(_) => println!("Cannot parse test case"),
                                     
                }   
            },
            Err(_) => println!("Cannot get # of test cases"),
        }
    }

    for t in 0..test_case {
        let n: u64 = std_iter.next().unwrap().unwrap().parse::<u64>().unwrap();
        println!("{}", get_sum_3_5(n));
    }
}

fn get_sum_3_5(n: u64) -> u64 {
    get_sum(n-1, 3) + get_sum(n-1, 5) - get_sum(n-1, 15)
}

fn get_sum(n:u64, k: u32) -> u64 {
    let p: u64 = n / (k as u64);
    (k as u64) * p * (p+1) / 2
}