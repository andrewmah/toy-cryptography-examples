use rand::Rng;

pub fn exp(g: u128, x: u128, p: u128) -> u128 {
    // returns g^x mod p using repeated squaring
    let mut odd_bits = 1;
    let mut base = g;
    let mut exponent = x;

    while exponent > 1 {
        if exponent % 2 == 0 {
            base = (base * base) % p;
            exponent = exponent / 2;
        } else {
            odd_bits = (odd_bits * base) % p;
            base = (base * base) % p;
            exponent = (exponent - 1) / 2;
        }
    }
    return (odd_bits * base) % p;
}

fn get_miller_rabin_parameters(p: u128) -> (u128, u128) {
    let mut s = 0;
    let mut d = p - 1;
    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }
    return (s, d);
}

pub fn is_prime(p: u128) -> bool {
    // even number causes value of s to equal 0 and messes up innner loop
    // by setting range to 0..-1
    if p % 2 == 0 {
        return false;
    }

    //runs 128 Miller Rabin prime tests on a number and returns if it is prime
    let reps = 128;
    let (s, d) = get_miller_rabin_parameters(p);
    let mut rng = rand::thread_rng();

    'witness_loop: for _i in 1..reps {
        let a = rng.gen_range(2..(p-1));
        let mut x = exp(a, d, p);

        if (x == 1) || (x == p - 1) {
            continue;
        }

        for _j in 0..(s-1) {
            x = (x * x) % p;
            if x == p - 1 {
                continue 'witness_loop;
            }
        }
        return false;
    }
    return true;

}

pub fn gen_safe_prime_pair(lower_bound:u128, upper_bound:u128) -> (u128, u128) {
    // returns two numbers primes p, q such that 
    // q = (p - 1) / 2
    // p = 7 mod 8
    
    // 2 generates a group of order q in the field Fp

    // p is know as a "safe prime"
    // large precomputed values of this can be found here
    // https://datatracker.ietf.org/doc/rfc3526/?include_text=1
    let new_lower_bound = if lower_bound % 2 == 0 {lower_bound + 1} else {lower_bound};

    for n in (new_lower_bound..upper_bound).step_by(2) {
        if (n % 3 == 0) || (n % 5 == 0) || (n % 7 == 0) {
            continue;
        }
        if is_prime(n) {
            let q = 2 * n + 1;
            if (q % 8 == 7) && is_prime(q) {
                return (n, q);
            }

            let q = (n - 1) / 2;
            if (n % 8 == 7) && is_prime(q) {
                return (q, n);
            }
        }
    }
    return (0, 0);
}


#[cfg(test)]
mod tests {
    // [TODO] set fixed random seed for tests
    use super::*;

    // pow tests
    #[test]
    fn test_pow_1() {
        assert_eq!(exp(6,3,13), 8);
    }
    #[test]
    fn test_pow_2() {
        assert_eq!(exp(62065077726107858, 696871303435469663, (1 << 31) - 1), 1432420130);
    }

    // is_prime tests
    #[test]
    fn test_is_prime_1() {
        assert!(is_prime(13));
    }
    #[test]
    fn test_is_prime_2() {
        assert!(is_prime(7919));
    }
    #[test]
    fn test_is_prime3() {
        assert!(is_prime((1 << 31) - 1));
    }
    #[test]
    fn test_is_prime_4() {
        assert!(!is_prime(20));
    }
    #[test]
    fn test_is_prime_5() {
        assert!(!is_prime(378899));
    }
    #[test]
    fn test_is_prime_6() {
       assert!(!is_prime((1 << 31) - 3));
    }   

    // gen_safe_prime_pair tests
    #[test]
    fn test_gen_safe_prime_pair_1() {
        let (p, q) = gen_safe_prime_pair(64, 128);
        println!("{}, {}", p, q);
        assert_eq!((p, q), (83, 167));
    }

    #[test]
    fn test_gen_safe_prime_pair_2() {
        let (p, q) = gen_safe_prime_pair(256, 512);
        println!("{}, {}", p, q);
        assert_eq!((p, q), (131, 263));
    }
}