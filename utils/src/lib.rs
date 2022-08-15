pub fn exp(g: u128, x: u128, p: u128) -> u128 {
    // returns g^x mod p using fast exponentation
    let mut odd_bits = 1;
    let mut base = g;
    let mut exponent = x;

    while exponent > 0 {
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