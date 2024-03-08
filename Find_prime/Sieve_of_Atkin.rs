use std::env;
// till limit using Sieve of Atkin 
fn sieve_of_atkin(limit: usize) {
    // Initialise the sieve array
    // with initial false values
    let mut sieve = vec![false; limit + 1];
    let mut num_of_prime = 0;

    let mut x:f32;
    let mut y:f32;
    

    // 2 and 3 are known to be prime
    if limit > 2 {
        sieve[2] = true;
    }
    if limit > 3 {
        sieve[3] = true;
    }

    /* Mark sieve[n] is true if one
    of the following is true:
    a) n = (4*x*x)+(y*y) has odd number of
    solutions, i.e., there exist
    odd number of distinct pairs (x, y)
    that satisfy the equation and
        n % 12 = 1 or n % 12 = 5.
    b) n = (3*x*x)+(y*y) has odd number of
    solutions and n % 12 = 7
    c) n = (3*x*x)-(y*y) has odd number of
    solutions, x > y and n % 12 = 11 */

    for x in 1..=limit {
        for y in 1..=limit {
            // Condition 1
            let mut n = (4 * x * x) + (y * y);
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n] ^= true;
            }

            // Condition 2
            n = (3 * x * x) + (y * y);
            if n <= limit && n % 12 == 7 {
                sieve[n] ^= true;
            }

            // // Condition 3
            // // n = (3 * x * x) - (y * y);
            // let x_squared = x.checked_mul(x).unwrap(); // Calculate x^2
            // let y_squared = y.checked_mul(y).unwrap(); // Calculate y^2
            
            // let n = match (3 * x_squared).checked_sub(y_squared) {
            //     Some(result) => result,
            //     None => continue, // Handle the overflow case appropriately
            // };
            
            // if x > y && n <= limit && n % 12 == 11 {
            //     sieve[n] ^= true;
            // }

            // condition 3
            if 3 * x * x >= y * y{
                n = (3 * x * x) - (y * y);
                if x > y && n <= limit && n % 12 == 11 {
                sieve[n] ^= true;
                }
            }
            

        }
    }

    // Mark all multiples
    // of squares as non-prime
    for r in 5..=(limit as f64).sqrt() as usize {
        if sieve[r] {
            for i in (r * r..=limit).step_by(r * r) {
                sieve[i] = false;
            }
        }
    }

    // Print primes using sieve[]
    for a in 1..=limit {
        if sieve[a] {
            num_of_prime += 1;

            // Print all prime number
            // println!("{}", a);
        }
    }

    println!("Number of prime number = {}", num_of_prime);
}

// Driver program
fn main() {

    // env
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let limit :usize = args[1].clone().parse().unwrap();
    
    println!("{}", limit );
    sieve_of_atkin(limit);
}
