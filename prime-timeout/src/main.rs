extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

const BIG_PRIME: u64 = 15485867;

fn is_prime(num: u64) -> bool {
    !(2..num).any(|i| num % i == 0)
}

fn main() {
    let pool = CpuPool::new_num_cpus();

    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);

        println!("Calculated primeness: {}", prime);
        let res: Result<bool, ()> = Ok(prime);
        res
    });

    println!("Created the future!");

    if prime_future.wait().unwrap() {
        println!("Prime!");
    } else {
        println!("Not Prime!");
    }
}
