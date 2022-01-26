

extern crate futures;
extern crate futures_cpupool;

use futures::Future; 
use futures_cpupool::CpuPool;

const BIG_PRIME: u64 = 15485867;
const BIG_PRIMES: [u64; 2] = [ 15485867, 32452843 ];

fn main() {

    println!("Sync version...");

	for i in 0..BIG_PRIMES.len() {
	    println!("{} {}", BIG_PRIMES[i], is_prime(BIG_PRIMES[i]));
    }

    println!("Async version...");

    // Create thread poool
    let pool = CpuPool::new_num_cpus(); 

    // spawn our computation, getting back a *future* of the answer
    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);

        // For reasons we'll see later, we need to return a Result here
        let res: Result<bool, ()> = Ok(prime);
        res
    });

    println!("Created the future");


}




fn is_prime(num: u64) -> bool {
	for i in 2..num {
		if num % i == 0 { 
			return false; 
		}
	}
	true
}