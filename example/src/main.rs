use encryption;

fn main() {
    let secret_key = encryption::gen();
    println!("secret key: {:x}\n", secret_key);

    loop {
        let mut message = String::new();
        println!("Enter a message: ");
        std::io::stdin().read_line(&mut message).unwrap();
        print!("\nmessage: {}", message);

        let (ciphertext, initial_vector) = encryption::enc(secret_key, &message);
        println!("ciphertext: ");
        for c in &ciphertext {
            println!("    {:x}, ", c);
        }

        let plaintext = encryption::dec(secret_key, &ciphertext, initial_vector);
        println!("decrypted plaintext: {}", plaintext);
    }
}
