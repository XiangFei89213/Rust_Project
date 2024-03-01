fn sieve(n:usize){
    
    println!("hello from function {}" , n);
    let prime: Vec<bool> = vec![true; n];
    
    println!("{:?}", prime);
     
    prime[2] =false;
    //println!("{:?}", prime);

    
}
fn main() {
    let n=50;
    let primes = sieve(n);
    println!("Hello, World!");
}
