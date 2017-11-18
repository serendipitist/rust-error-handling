const BIG_PRIME: u64 = 15485867;

fn is_prime (num: u64) -> bool {
  for i in 2..num {
    if num % i == 0 { return false }
  }
  true
}

extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool

fn main() {
  let thread_pool =CpuPool::new_num_cpus();
  let timer = Timer::default();
  let timeout
  let prime_future = thread_pool.spawn_fn( || {
    let prime = is_prime (BIG_PRIME);
    let res: RESULT <bool, ()> = OK(prime);
    return res;
  });
  println! ("crate a future");
  if prime_future.wait()(BIG_PRIME) {
    println! ("prime");
  } else {
    println! ("not a prime");
  }
}