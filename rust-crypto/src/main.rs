use ed25519_dalek::{SigningKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use hex;

fn main() {
    // Step 1: Generate a signing key using a cryptographically secure random number generator
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);

    // Step 2: Derive the verifying key (public key) from the signing key
    let verifying_key = signing_key.verifying_key();

    // Step 3: Define the message to be signed
    let message = b"This is a test of the tsunami alert system.";

    // Step 4: Sign the message
    let signature: Signature = signing_key.sign(message);
    println!("Message: {}", String::from_utf8_lossy(message));
    println!("Signature: {}", hex::encode(signature.to_bytes()));

    // Step 5: Print the signing key (secret key)
    println!("Signing (Secret) Key: {}", hex::encode(signing_key.to_bytes()));

    // Step 6: Print the verifying key (public key)
    println!("Verifying (Public) Key: {}", hex::encode(verifying_key.to_bytes()));

    // Step 7: Verify the signature using the signing key
    assert!(signing_key.verify(message, &signature).is_ok());
    println!("Signature verified using the signing key!");

    // Step 8: Verify the signature using the verifying (public) key
    assert!(verifying_key.verify(message, &signature).is_ok());
    println!("Signature verified using the verifying key!");
}

