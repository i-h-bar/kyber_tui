mod keys;

use keys::{KeyPair, KeyError};
use crate::keys::public::Public;

fn main() {
    match run_example() {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_example() -> Result<(), KeyError> {
    let bob = KeyPair::generate()?;
    let alice = KeyPair::generate()?;

    let bob_pub = bob.public.to_b64();
    let bob_pub_obj = Public::from_b64(&bob_pub)?;

    let plaintext = "Hello Bob! This is a secret message protected by post-quantum cryptography!";

    let encrypted = alice.encrypt_b64(plaintext, &bob.public)?;

    let decrypted = bob.decrypt_b64(&encrypted, &alice.public)?;
    println!("   Decrypted: {:?}\n", decrypted);

    assert_eq!(plaintext, decrypted);

    Ok(())
}
