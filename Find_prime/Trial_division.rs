use std::f64;

// Function to check if a number is a prime number or not
fn trial_division(n: usize) -> bool {
    // Initializing with the value 2 from where the number is checked
    let mut i = 2;

    // Computing the square root of the number N
    let k = (n as f64).sqrt().ceil() as usize;

    // While loop till the square root of N
    while i <= k {
        // If any of the numbers between [2, sqrt(N)] is a factor of N
        // Then the number is composite
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    // If none of the numbers is a factor, then it is a prime number
    true
}

// Driver code
// fn main() {
//     // let mut num_of_prime = 0;
//     // let prime = 1000;
//     let numbers = [2, 3, 5, 6, 7, 11, 13, 17, 19, 23];
    
//     // for n in 1..prime {
//     //     if trial_division(n) {
//     //         num_of_prime+=1;
//     //         //print!("{} ", n);
//     //     }
//     // }

//     println!("Number of prime numbers: ");
//     for &num in numbers.iter() {
//         if trial_division(num) {
//             print!("{}", num)
//         }
//     }
    
//     // println!("number of prime : {}", num_of_prime);
// }

fn main() {
    let numbers = [2, 3, 5, 6, 7, 11, 13, 17, 19, 23];

    println!("Prime numbers in the array:");
    for &num in numbers.iter() {
        if trial_division(num) {
            print!("{} ", num);
        }
        else if num == 2 {
            print!("{} ", &num);
        }
            
    }
    println!(); // Print a newline after printing all prime numbers
}