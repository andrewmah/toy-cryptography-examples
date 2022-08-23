use utils::exp;

//one way function (OWF)
fn base_one_way_fn(x: u128) -> u128 {
    let p: u128 = 2147483783; //prime mod
    let g: u128 = 2; //generates group of order 1073741891 

    return exp(g, x, p);
}

pub fn one_way_fn(x: u128) -> u128 {
    // modified OWF used for Goldreich Levin Theorem
    //[TODO check this bit fiddling]
    let y1 = base_one_way_fn(x >> 64);
    let y2 = ((x) as u64) as u128;

    return (y1 << 64) | y2;
}


// hardcore predicate of a one way function
pub fn hcp(x: u128) -> u128 {
    // f is a one way function
    // x is input
    // returns the hardcore predicate bit of f(x)

    // split input
    let x1 = (x >> 64) as u64;
    let x2 = (x) as u64;

    // take inner product by taking bitwise AND then counting set bits
    let mut prod = x1 & x2;
    let mut count = 0; // accumulate total set bits 

    while prod != 0 {
        prod &= prod - 1; 
        count += 1 
    }
    return count % 2
}

//pseudo random generator (PRG)
pub fn prg(x: u128) -> (u128, u128) {
    let mut s = x;
    let mut g1 = 0;
    let mut g2 = 0;
    for _i in 0..128 {
        s = one_way_fn(s);
        g1 = g1 << 1;
        g1 = g1 | hcp(s);
    }
    for _i in 0..128 {
        s = one_way_fn(s);
        g2 = g2 << 1;
        g2 = g2 | hcp(s);
    }
    return (g1, g2);
}

//pseudo random function (PRF)
pub fn prf(key: u128, x: u128) -> u128 {
    let mut output = x;
    let mut key_bits = key;
    for _i in 0..128 {
        let (g0, g1) = prg(output);
        if (key_bits & 0x1 == 0) {
            output = g0;
        }
        else {
            output = g1;
        }
    }
    return output;
}