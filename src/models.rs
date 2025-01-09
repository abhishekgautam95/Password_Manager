use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordEntry {
    pub name: String,
    pub encrypted_password: Vec<u8>,
    pub nonce: Vec<u8>,
}