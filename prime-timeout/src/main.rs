extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

use std::time::Duration;
use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;

const BIG_PRIME: u64 = 15485867;

fn is_prime(num: u64) -> bool {
    !(2..num).any(|i| num % i == 0)
}

fn main() {
    let pool = CpuPool::new_num_cpus();
    let timer = Timer::default();

    let timeout = timer.sleep(Duration::from_millis(750)).then(|_| Err(()));

    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);

        let res: Result<bool, ()> = Ok(prime);
        res
    });

    let winner = timeout.select(prime_future).map(|(win, _)| win);

    match winner.wait() {
        Ok(primeness) => println!("Prime? {}", primeness),
        Err(_) => println!("Timed out")
    }
}
