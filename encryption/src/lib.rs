use rand::Rng;
use primitives;

pub fn gen() -> u128 {
    // generate a uniformly random u128 secret key
    let mut rng = rand::thread_rng();
    let secret_key:u128 = rng.gen_range(0..u128::MAX);
    return secret_key;
}

pub fn enc(sk: u128, message:&String) -> (Vec<u128>, u128) {
    // encrypt a message m with secret key sk
    // returns random initial vector and ciphertext

    // generate initial vector
    let mut rng = rand::thread_rng();
    let initial_vector:u128 = rng.gen_range(0..u128::MAX);

    let mut ciphertext_blocks: Vec<u128> = Vec::new();
    let mut buffer: [u8; 16] = [0; 16];

    // break string into list of ascii chars one bytes each
    // encrypt 16 chars at a time because prf pad is u128
    // 128 / 8 = 16
    for (i, c) in message.as_bytes().iter().enumerate() {
        buffer[i % 16] = *c;

        // u128 / u8 = 16
        if i % 16 == 15 {
            let counter = initial_vector + ((i / 16) as u128);
            let pad = primitives::prf(sk, counter);
            let buffer_chars = u128::from_be_bytes(buffer);
            ciphertext_blocks.push(pad ^ buffer_chars);
            
            buffer = [0; 16];
        }
    }
   
    return (ciphertext_blocks, initial_vector);
}

pub fn dec(sk:u128, ciphertext:&Vec<u128>, initial_vector:u128) -> String {
    let mut plaintext = String::new();
    for (i, c) in ciphertext.iter().enumerate() {
        let counter = initial_vector + (i as u128);
        let pad = primitives::prf(sk, counter);
        let plaintext_block = pad ^ c;
        for ascii_char in plaintext_block.to_be_bytes() {
            plaintext.push(ascii_char as char);
        }
    }
    return plaintext;
}





#[cfg(test)]
mod tests {
    // [TODO] set fixed random seed for tests
    use super::*;

    #[test]
    fn test_correctness_1() {
        let message = String::from("Hello, World!123");
        
        let sk = gen();
        let (ciphertext, initial_vector) = enc(sk, &message);
        let plaintext = dec(sk, &ciphertext, initial_vector);

        assert_eq!(message, plaintext);
    }
}