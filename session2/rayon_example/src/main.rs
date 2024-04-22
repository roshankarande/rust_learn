use std::time::Instant;

use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
    // (2 ..= n/2).into_par_iter().all(|i| n % i != 0 )
}

fn main() {

    let numbers: Vec<u32> = (0..100000).collect();
    let now = Instant::now();
    let mut primes: Vec<&u32> = numbers.par_iter().filter(|n| is_prime(**n)).collect();
    // let mut primes: Vec<u32> = numbers.into_iter().filter(|n| is_prime(*n)).collect();
    primes.par_sort_unstable();
    
    let elapsed = now.elapsed();

    println!("{} :: {} secs", primes.len(), elapsed.as_secs_f32());

}
