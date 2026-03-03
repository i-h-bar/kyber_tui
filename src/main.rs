mod keys;

use pqc_kyber::*;
use pqcrypto_sphincsplus::sphincsshake128fsimple;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use rand::rngs::OsRng;

fn main() {
    match run_example() {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_example() -> Result<(), String> {
    println!("=== Post-Quantum Cryptography Example ===\n");

    // ========================================
    // STEP 1: Key Generation
    // ========================================
    println!("1. Generating keys...");

    // Use OS's cryptographically secure random number generator
    let mut rng = OsRng;

    // Bob generates a Kyber keypair for key exchange
    let bob_kem_keys = keypair(&mut rng)
        .map_err(|e| format!("Kyber keypair generation failed: {:?}", e))?;
    println!("   ✓ Bob's Kyber KEM keypair generated");

    // Alice generates a SPHINCS+ keypair for signing
    let (alice_verify_key, alice_signing_key) = sphincsshake128fsimple::keypair();
    println!("   ✓ Alice's SPHINCS+ signing keypair generated\n");

    // ========================================
    // STEP 2: Key Exchange (Kyber)
    // ========================================
    println!("2. Alice encapsulates shared secret using Bob's public key...");

    // Alice uses Bob's public key to create a shared secret
    let (kyber_ciphertext, shared_secret_alice) = encapsulate(&bob_kem_keys.public, &mut rng)
        .map_err(|e| format!("Encapsulation failed: {:?}", e))?;
    println!("   ✓ Shared secret encapsulated");
    println!("   ✓ Kyber ciphertext size: {} bytes\n", kyber_ciphertext.len());

    // ========================================
    // STEP 3: Encrypt Message (AES-256-GCM)
    // ========================================
    println!("3. Alice encrypts a message using the shared secret...");

    let plaintext = b"Hello Bob! This is a secret message protected by post-quantum cryptography!";
    println!("   Original message: {:?}", String::from_utf8_lossy(plaintext));

    // Create AES-256-GCM cipher from the shared secret
    let cipher_alice = Aes256Gcm::new_from_slice(&shared_secret_alice)
        .map_err(|e| format!("Failed to create cipher: {:?}", e))?;

    // Generate a random nonce (12 bytes for AES-GCM)
    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the message
    let ciphertext = cipher_alice.encrypt(nonce, plaintext.as_ref())
        .map_err(|e| format!("Encryption failed: {:?}", e))?;
    println!("   ✓ Message encrypted with AES-256-GCM");
    println!("   ✓ Ciphertext size: {} bytes\n", ciphertext.len());

    // ========================================
    // STEP 4: Sign the Ciphertext (SPHINCS+)
    // ========================================
    println!("4. Alice signs the ciphertext with SPHINCS+...");

    // Alice signs the ciphertext to prove authenticity
    let signature = sphincsshake128fsimple::sign(&ciphertext, &alice_signing_key);
    println!("   ✓ Signature created");
    println!("   ✓ Signature size: {} bytes\n", signature.len());

    // ========================================
    // TRANSMISSION SIMULATION
    // ========================================
    println!("📡 Transmitting to Bob:");
    println!("   - Kyber ciphertext (for key exchange)");
    println!("   - Encrypted message");
    println!("   - Nonce");
    println!("   - SPHINCS+ signature");
    println!("   - Alice's public verification key\n");

    // ========================================
    // STEP 5: Verify Signature (Bob's Side)
    // ========================================
    println!("5. Bob verifies Alice's signature...");

    // Bob verifies the signature using Alice's public key
    match sphincsshake128fsimple::open(&signature, &alice_verify_key) {
        Ok(verified_msg) => {
            // Check that the verified message matches the ciphertext
            if verified_msg == ciphertext {
                println!("   ✓ Signature verified! Message is authentic.\n");
            } else {
                return Err("Signature valid but message mismatch!".to_string());
            }
        }
        Err(_) => {
            println!("   ✗ Signature verification failed!");
            return Err("Invalid signature".to_string());
        }
    }

    // ========================================
    // STEP 6: Decapsulate Shared Secret (Bob)
    // ========================================
    println!("6. Bob decapsulates the shared secret...");

    // Bob uses his private key to recover the shared secret
    let shared_secret_bob = decapsulate(&kyber_ciphertext, &bob_kem_keys.secret)
        .map_err(|e| format!("Decapsulation failed: {:?}", e))?;
    println!("   ✓ Shared secret recovered");

    // Verify both secrets match
    assert_eq!(shared_secret_alice, shared_secret_bob);
    println!("   ✓ Shared secrets match!\n");

    // ========================================
    // STEP 7: Decrypt Message (Bob)
    // ========================================
    println!("7. Bob decrypts the message...");

    // Create cipher from the recovered shared secret
    let cipher_bob = Aes256Gcm::new_from_slice(&shared_secret_bob)
        .map_err(|e| format!("Failed to create cipher: {:?}", e))?;

    // Decrypt the message
    let decrypted_plaintext = cipher_bob.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decryption failed: {:?}", e))?;
    println!("   ✓ Message decrypted successfully");
    println!("   Decrypted message: {:?}\n", String::from_utf8_lossy(&decrypted_plaintext));

    // Verify decryption worked
    assert_eq!(plaintext.as_slice(), decrypted_plaintext.as_slice());

    // ========================================
    // SUMMARY
    // ========================================
    println!("=== ✓ Success! ===");
    println!("Post-quantum secure communication established:");
    println!("  • Kyber-512 (lattice-based KEM) - Key exchange");
    println!("  • SPHINCS+-SHAKE128f (hash-based) - Digital signatures");
    println!("  • AES-256-GCM - Symmetric encryption");
    println!("\nAll operations are quantum-resistant! 🔒");

    Ok(())
}
