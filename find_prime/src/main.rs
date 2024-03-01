fn sieve(n:usize){
    
    println!("hello from function {}" , n);
    let mut prime: Vec<bool> = vec![true; n];
    
    //mutable arrray
    // println!("{:?}", prime);
     
    // prime[2] =false;
    // println!("{:?}", prime);
    
    // for loop
    let mut i:usize =2;
    let mut p:i32 ;
    let mut done = false;
    
    while !done{
        let i = i+1;
        
        if prime[i] == true{
            for p in (i*i) .. n {
                prime[i] = false;
            }
        }
        
        if i*i <= n {
            done = true;
        }
    }

    // print result
    for p in 2..n+1{
        if prime[p] == true{
            println!("{}" , p)
        }
    }
    
}
fn main() {
    let n=50;
    let primes = sieve(n);
    println!("Hello, World!");
}
