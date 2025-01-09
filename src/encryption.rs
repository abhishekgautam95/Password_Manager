use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;

pub fn derive_key(master_password: &str) -> [u8; 32] {
    let salt = b"randomsalt"; // Use a unique salt in production
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .expect("Failed to derive key");
    key
}

pub fn encrypt_password(key: &[u8; 32], password: &str) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Failed to create cipher");
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
    let encrypted_password = cipher
        .encrypt(&nonce, password.as_bytes())
        .expect("Failed to encrypt password");
    (encrypted_password, nonce.to_vec())
}

pub fn decrypt_password(key: &[u8; 32], encrypted_password: &[u8], nonce: &[u8]) -> String {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Failed to create cipher");
    let nonce = Nonce::from_slice(nonce);
    let decrypted_password = cipher
        .decrypt(nonce, encrypted_password)
        .expect("Failed to decrypt password");
    String::from_utf8(decrypted_password).expect("Invalid UTF-8")
}