// use std::env;
// use std::io;
// extern crate rand; 
// use rand::{thread_rng, Rng};

// // Utility function to do modular exponentiation.
// // It returns (x^y) % p

// fn power(x: u64, y: u64, p: u64) -> u64 {
//     let mut res = 1;
//     let mut x = x % p;
//     let mut y = y;

//     while y > 0 {
//         if y & 1 == 1 {
//             res = (res * x) % p;
//         }
//         y >>= 1;
//         x = (x * x) % p;
//     }
//     res
// }

// // This function is called for all k trials. It returns
// // false if n is composite and returns true if n is
// // probably prime.
// // d is an odd number such that d*2 = n-1
// // for some r >= 1
// fn miller_test(mut d: u64, n: u64) -> bool {
//     let min=2;
//     let max=n-3;
//     let mut a = rand::random::<u64>();
//     a = a%(max-min+1)+min;
//     let mut x = power(a, d, n);

//     if x == 1 || x == n - 1 {
//         return true;
//     }

//     while d != n - 1 {
//         x = (x * x) % n;
//         d *= 2;

//         if x == 1 {
//             return false;
//         }
//         if x == n - 1 {
//             return true;
//         }
//     }
//     false
// }

// // It returns false if n is composite and returns true if n
// // is probably prime. k is an input parameter that determines
// // accuracy level. Higher value of k indicates more accuracy.
// fn is_prime(n: u64, k: u64) -> bool {
//     if n <= 1 || n == 4 {
//         return false;
//     }
//     if n <= 3 {
//         return true;
//     }

//     let mut d = n - 1;
//     while d % 2 == 0 {
//         d /= 2;
//     }

//     for _ in 0..k {
//         if !miller_test(d, n) {
//             return false;
//         }
//     }
//     true
// }

// fn generate_large_prime(limit: u64) -> u64 {
//     let mut rng = rand::thread_rng();
    
//     loop {
//         let candidate: u64 = rng.gen_range(2..=limit);
//         if is_prime(candidate, 4) {
//             return candidate;
//         }
//     }
    
// }

// fn main() {
//     env::set_var("RUST_BACKTRACE", "1");
//     // let mut rng = rand::thread_rng();

//     // let n1: u8 = rng.gen();
//     // let n2: u16 = rng.gen();

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     let limit: u64 = input.trim().parse().expect("Please enter a valid number");


//     // let limit: u64 = 1_000_000_000_000_000_000;
//     let prime = generate_large_prime(limit);
//     println!("Generated prime: {}", prime);
    
    
// /*
//     let k = 4; // Number of iterations
//     let mut num_of_prime = 0;
//     let prime = 100;

//     for n in 2..prime {
//         if is_prime(n as u64, k as u64) {
//             num_of_prime += 1;
//         }
//     }

//     println!("{}", num_of_prime);
// */
    
// }

use std::env;
use std::io;
use rand::{thread_rng, Rng};

// Utility function to do modular exponentiation.
// It returns (x^y) % p
fn power(x: u64, y: u64, p: u64) -> u64 {
    let mut res: u128 = 1;
    let mut x = x as u128 % p as u128;
    let mut y = y;
    let p = p as u128;

    while y > 0 {
        if y & 1 == 1 {
            res = (res * x) % p;
        }
        y >>= 1;
        x = (x * x) % p;
    }
    res as u64
}

// This function is called for all k trials. It returns
// false if n is composite and returns true if n is
// probably prime.
// d is an odd number such that d*2^r = n-1
// for some r >= 1
fn miller_test(mut d: u64, n: u64) -> bool {
    let min = 2;
    let max = n - 3;
    let mut rng = thread_rng();
    let a = rng.gen_range(min..=max);
    let mut x = power(a, d, n);

    if x == 1 || x == n - 1 {
        return true;
    }

    let n128 = n as u128;
    while d != n - 1 {
        x = ((x as u128 * x as u128) % n128) as u64;
        d *= 2;

        if x == 1 {
            return false;
        }
        if x == n - 1 {
            return true;
        }
    }
    false
}

// It returns false if n is composite and returns true if n
// is probably prime. k is an input parameter that determines
// accuracy level. Higher value of k indicates more accuracy.
fn is_prime(n: u64, k: u64) -> bool {
    if n <= 1 || n == 4 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    for _ in 0..k {
        if !miller_test(d, n) {
            return false;
        }
    }
    true
}

fn generate_large_prime(limit: u64) -> u64 {
    let mut rng = thread_rng();
    
    loop {
        let candidate: u64 = rng.gen_range(2..=limit);
        if is_prime(candidate, 4) {
            return candidate;
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    // let limit: u64 = input.trim().parse().expect("Please enter a valid number");

    let limit: u64 = 1000;
    let prime = generate_large_prime(limit);
    println!("Generated prime: {}", prime);
}
