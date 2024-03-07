// Prints the count of prime numbers smaller than n
fn sieve_of_sundaram(n: usize) {
    // In general Sieve of Sundaram, produces primes smaller
    // than (2*x + 2) for a number given number x.
    // Since we want primes smaller than n, we reduce n to half
    let n_new = (n - 1) / 2;
    let mut num_of_prime = 0;

    // This array is used to separate numbers of the form i+j+2ij
    // from others where 1 <= i <= j
    let mut marked = vec![false; n_new + 1];

    // Main logic of Sundaram. Mark all numbers of the
    // form i + j + 2ij as true where 1 <= i <= j
    for i in 1..=n_new {
        for j in i..=((n_new - i) / (2 * i + 1)) {
            marked[i + j + 2 * i * j] = true;
        }
    }

    // two is also a prime number 
    num_of_prime +=1 ;

    // Count the prime numbers
    for i in 1..=n_new {
        if !marked[i] {
            num_of_prime += 1;

            //print number
            // print!("{} ", 2 * i + 1);
        }
    }
    
    println!("Number of prime numbers = {}", num_of_prime);
}

// Driver program to test above
fn main() {
    let n =500000000;
    sieve_of_sundaram(n);
}
