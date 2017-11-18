const BIG_PRIME: u64 = 15485867;

fn is_prime (num: u64) -> bool {
  for i in 2..num {
    if num % i == 0 { return false }
  }
  true
}

fn main() {
  if is_prime(BIG_PRIME) {
    println! ("prime");
  } else {
    println! ("not a prime");
  }
}