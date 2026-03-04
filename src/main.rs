mod keys;

use keys::{KeyPair, KeyError};

fn main() {
    match run_example() {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_example() -> Result<(), KeyError> {
    let bob = KeyPair::generate()?;
    let alice = KeyPair::generate()?;

    let plaintext = "Hello Bob! This is a secret message protected by post-quantum cryptography!";

    let encrypted = alice.encrypt_b64(plaintext, &bob.public)?;

    let decrypted = bob.decrypt_b64(&encrypted, &alice.public)?;
    println!("   Decrypted: {:?}\n", decrypted);

    assert_eq!(plaintext, decrypted);

    Ok(())
}
