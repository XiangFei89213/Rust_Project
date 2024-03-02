 fn sieve(n:usize){
    
    println!("hello from function {}" , n);
    let mut prime: Vec<bool> = vec![true; n];
    
    //mutable arrray
    // println!("{:?}", prime);
     
    // prime[2] =false;
    // println!("{:?}", prime);
    
    // for loop
    let mut i:usize =2;
    let mut p:usize ;
    let mut done = false;
    
    while !done{
        
        
        println!("when i={}" , i);
        
        if prime[i] == true{
          
          print!(" prime = {}" , prime[i]);
          
          let mut end = false;
          p=i*i;
          while !end{
                prime[p] = false;
                println!("change prime{} to {}" , p, prime[p]);
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
        println!(" i={}", i);
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
