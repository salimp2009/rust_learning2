use ::std::time::Instant;

use rand::Rng;

pub fn generate_random_values(count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::with_capacity(count);

    (0..count).for_each(|_| ret.push(rng.gen_range(i64::MIN..i64::MAX)));
    ret
}

fn main() {
    println!("Hello, world!");
}
