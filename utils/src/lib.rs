use rand::Rng;

pub fn exp(g: u128, x: u128, p: u128) -> u128 {
    // returns g^x mod p using fast exponentation
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
    //runs 128 Miller Rabin prime tests on a number and returns if it is prime
    let reps = 10;
    let (s, d) = get_miller_rabin_parameters(p);
    let mut rng = rand::thread_rng();

    println!("s={}, d={}", s, d);

    for _i in 1..reps {
        println!("i={}", _i);
        let a = rng.gen_range(2..(p-1));
        let mut x = exp(a, d, p);
        let mut break_flag = false;

        println!("a={}, d={}, p={}", a, d, p);
        println!("x={}, p-1={}", x, p-1);
        if (x == 1) || (x == p - 1) {
            continue;
        }

        for _j in 0..(s-1) {
            x = (x * x) % p;
            if x == p - 1 {
                break_flag = true;
                break;
            }
        }
        if break_flag {
            continue;
        }
        return false;
    }
    return true;

}

// pub fn gen_prime(bits: u32) -> u128 {

// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(exp(6,3,13), 8);

        assert_eq!(exp(62065077726107858, 696871303435469663, (1 << 31) - 1), 1432420130);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(13));
        assert!(is_prime((1 << 31) - 1));

        assert!(!is_prime((1 << 31) - 3));
    }   
}