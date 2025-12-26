#[test]
fn test() {
    assert_eq!(23, solution3(10));    
    assert_eq!(233168, solution3(1000));
    assert_eq!(233333333166666668, solution3(1000000000));
}


fn multiplies_3_5(n: u64) -> u64 
{
    let mut sum: u64 = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as u64;
        }
    }

    println!("{}, {}", n, sum);
    sum
}

fn multiplies_3_5_2(n: u64, k: u32) -> u64 {
    let mut sum: u64 = 0;
    let mut temp: u64 = 0;
    loop {
        temp += k as u64;
        if temp >= n {break;}
        sum += temp;
    }

    sum
}

fn multiplies_3_5_3(n: u64, k: u32) -> u64 {
    let p = n / k as u64;
    k as u64 * p*(p+1)/2
}

fn solution3(n:u64) -> u64 {
    multiplies_3_5_3(n-1, 3) + multiplies_3_5_3(n-1, 5) - multiplies_3_5_3(n-1, 15)
}

fn solution2(n: u64) -> u64 {
       multiplies_3_5_2(n, 3) + multiplies_3_5_2(n, 5) - multiplies_3_5_2(n, 15)
}