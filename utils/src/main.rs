use utils;

fn main() {
    let (p, q) = utils::gen_safe_prime_pair(1 << 30, 1 << 32);
    println!("p = {}, q = {}", p, q);
}