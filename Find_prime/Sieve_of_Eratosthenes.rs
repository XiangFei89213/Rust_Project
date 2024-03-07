use std::env;
fn sieve(n:usize){
    
    // println!("hello from function {}" , n);
    let mut prime: Vec<bool> = vec![true; n];
    
    // for loop
    let mut i:usize =2;
    let mut p:usize ;
    let mut done = false;
    
    while !done{
        
        
        // println!("when i={}" , i);
        
        if prime[i] == true{
          
        //   print!(" prime = {}" , prime[i]);
          
          let mut end = false;
          p=i*i;
          while !end{
                prime[p] = false;
                // println!("change prime{} to {}" , p, prime[p]);
                p = p+i;
                
                if p>= n{
                  end = true;
                }
          }

        }
        
        if i*i >= n {
            done = true;
        }
        
        i = i+1;
        //println!(" i={}", i);
    }

    // print result
    let mut num =0;
    for p in 2..n{
        if prime[p] == true{
            num = num+1;
            //println!("{}" , p)
        }
    }
    println!("there are {} prime number" , num);

    
}
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let n=500000000;
    //let primes = sieve(n);
    sieve(n);
    println!("Hello, World!");
}
